from .spacerock import SpaceRock
from .units import Units
from .utils import great_circle_distance, time_handler

from astropy.coordinates import Angle
import numpy as np
import warnings

class MPChecker:

    def __init__(self, catalog='mpcorb_extended', update=False):
        self.rocks = self.load_rocks(catalog=catalog, update=update)
    
    def load_rocks(self, catalog, update):
        from .paths import MPC_PATH
        import pathlib
        rocksfile = pathlib.Path(MPC_PATH + f'/{catalog}.json.gz')
        if rocksfile.is_file() and update == False:
            rocks = SpaceRock.from_mpc(f'{catalog}', download_data=False, metadata='Orbit_type')
        else:
            rocks = SpaceRock.from_mpc(f'{catalog}', download_data=True, metadata='Orbit_type')
        return rocks
    
    def check(self, ra, dec, radius, epoch, maglim, obscode, units=Units()):

        epoch = time_handler(epoch, units)
        ra = Angle(ra, units.ra)
        dec = Angle(dec, units.dec)
        radius = Angle(radius, units.angular_separation)
        
        prop = self.rocks.analytic_propagate(epoch=epoch)
        obs = prop.observe(obscode=obscode)
        arc_dis = great_circle_distance(obs.ra, obs.dec, ra, dec)
        if radius.deg > 3:
            warnings.warn('Exceed maximum search radius, using radius = 3 degrees instead.')
        in_field_radius = max([3*np.pi/180, 5*radius.rad])
        in_field = arc_dis < in_field_radius
        
        if in_field.sum() > 0:
            rocks = self.rocks[in_field]
            units = Units()
            units.timescale = 'utc'
            prop, _, _ = rocks.propagate(epochs=epoch, model='PLANETS', units=units)
            obs = prop.observe(obscode=obscode)
            arc_dis = great_circle_distance(obs.ra, obs.dec, ra, dec)
            in_field = arc_dis < radius.rad
            bright = obs.mag < maglim
            gotcha = in_field * bright
            if gotcha.sum() > 0: 
                return rocks[gotcha], obs[gotcha]
            else:
                warnings.warn('No known object! Return None.')
                return None, None        
        else:
            warnings.warn('No known object! Return None.')
            return None, None