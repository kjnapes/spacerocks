###############################################################################
# SpaceRocks, version 0.6.6
#
# Author: Kevin Napier kjnapier@umich.edu
################################################################################

import sys
import os
import random

import healpy as hp

import rebound
import reboundx
from reboundx import constants

import numpy as np
import pandas as pd
from numba import jit
from scipy.optimize import newton
import ephem
from skyfield.api import Topos, Loader

from astropy import units as u
from astropy.table import Table
from astropy.coordinates import Angle
from astropy.time import Time
from astropy.coordinates import SkyCoord

import matplotlib.pyplot as plt
import matplotlib.ticker as mticker
import cartopy.crs as ccrs

from .linalg3d import *
from .constants import *
#from .jacobians import *

# Read in the observatory codes file and rehash as a dataframe.
observatories = pd.read_csv(os.path.join(os.path.dirname(__file__), 'data', 'observatories.csv'))

# Load in planets for ephemeride calculation.
load = Loader('./Skyfield-Data', expire=False, verbose=False)
ts = load.timescale()
planets = load('de423.bsp')
sun = planets['sun']


class SpaceRock:

    def __init__(self, input_coordinates='keplerian', input_frame='barycentric',
                 input_angles='degrees', input_time_format='jd',
                 input_time_scale='utc', precise=False, NSIDE=None, obscode=None,
                 uncertainties=None, *args, **kwargs):

        self.frame = input_frame
        self.tau = Time(kwargs.get('tau'),
                        format=input_time_format,
                        scale=input_time_scale).jd * u.day

        if obscode is not None:
            self.obscode = str(obscode).zfill(3)
            obs = observatories[observatories.obscode == self.obscode]
            self.obslat = obs.lat.values
            self.obslon = obs.lon.values
            self.obselev = obs.elevation.values
        else:
            self.obscode = None

        if self.frame == 'barycentric':
            mu = mu_bary
        elif self.frame == 'heliocentric':
            mu = mu_helio

        self.precise = precise

        if (self.precise is not True) and (self.precise is not False):
            raise ValueError('The parameter precise must be set to either True or False.')

        # Case-insensitive keyword arguments.
        kwargs = {key.lower(): data for key, data in kwargs.items()}
        keywords = ['a', 'e', 'inc', 'node', 'arg', 'M',
                    'x', 'y', 'z', 'vx', 'vy', 'vz',
                    'tau', 'epoch', 'h', 'name']

        if not all(key in keywords for key in [*kwargs]):
            raise ValueError('Keywords are limited to a, e, inc, node,\
                              arg, M, x, y, z, vx, vy, vz, tau, epoch,\
                              H, name')

        input_coordinates = input_coordinates.lower()
        input_frame = input_frame.lower()
        input_angles = input_angles.lower()

        # scalar input -> arrays
        for idx, key in enumerate([*kwargs]):
            if np.isscalar(kwargs.get(key)):
                kwargs[key] = np.array([kwargs.get(key)])

        if NSIDE is not None:
            if np.isscalar(NSIDE):
                NSIDE = np.array([NSIDE])
            self.NSIDE = NSIDE
        else:
            self.NSIDE = None

        if input_angles == 'degrees':
            angle_unit = u.degree
        elif input_angles == 'radians':
            angle_unit = u.radian
        else:
            raise ValueError('The input_angles argument must be a string \
                              that reads either degrees or radians.')

        if input_coordinates == 'keplerian':

            self.a = kwargs.get('a') * u.au
            self.e = kwargs.get('e') * u.dimensionless_unscaled
            self.inc = Angle(kwargs.get('inc'), angle_unit).to(u.rad)
            self.node = Angle(kwargs.get('node'), angle_unit).to(u.rad)
            self.arg = Angle(kwargs.get('arg'), angle_unit).to(u.rad)

            if (kwargs.get('epoch') is None) and (kwargs.get('M') is not None):
                self.M = Angle(kwargs.get('M'), angle_unit) * u.rad
                lp = self.M < np.pi * u.rad
                self.epoch[lp] = self.tau.jd[lp] * u.day - self.M[lp] / np.sqrt(mu_bary / self.a[lp]**3)
                self.epoch[~lp] = self.tau.jd[~lp] * u.day + (2*np.pi * u.rad - self.M[~lp]) / np.sqrt(mu_bary / self.a[~lp]**3)
                self.epoch = Time(self.epoch, format='jd', scale='utc')
                #self.epoch = self.tau - self.M / np.sqrt(mu / self.a**3)

            elif (kwargs.get('M') is None) and (kwargs.get('epoch') is not None):
                # self.epoch = kwargs.get('epoch') * u.day # time at perihelion
                self.epoch = Time(kwargs.get('epoch'),
                                  format=input_time_format,
                                  scale=input_time_scale)
                self.M = np.sqrt(mu / self.a**3) * (self.tau - self.epoch.jd * u.day)

            # this looks redundant but it allows for broadcasring.
            self.tau = self.epoch +  self.M / np.sqrt(mu / self.a**3)
            self.kep_to_xyz(mu)

            if self.frame == 'barycentric':
                self.xyz_to_equa()

            elif self.frame == 'heliocentric':
                self.to_bary()
                self.xyz_to_equa()
                self.to_helio()

        elif input_coordinates == 'cartesian':

            self.x = kwargs.get('x') * u.au
            self.y = kwargs.get('y') * u.au
            self.z = kwargs.get('z') * u.au
            self.vx = kwargs.get('vx') * (u.au / u.day)
            self.vy = kwargs.get('vy') * (u.au / u.day)
            self.vz = kwargs.get('vz') * (u.au / u.day)

            self.xyz_to_kep(mu)
            lp = self.M < np.pi * u.rad
            self.epoch[lp] = self.tau.jd[lp] * u.day - self.M[lp] / np.sqrt(mu_bary / self.a[lp]**3)
            self.epoch[~lp] = self.tau.jd[~lp] * u.day + (2*np.pi * u.rad - self.M[~lp]) / np.sqrt(mu_bary / self.a[~lp]**3)
            self.epoch = Time(self.epoch, format='jd', scale='utc')
            #self.epoch = self.tau - self.M / np.sqrt(mu / self.a**3)

            # this looks redundant but it allows for broadcasring.
            self.tau = self.epoch + self.M / np.sqrt(mu / self.a**3)

            if self.frame == 'barycentric':
                self.xyz_to_equa()

            elif self.frame == 'heliocentric':
                self.to_bary()
                self.xyz_to_equa()
                self.to_helio()

        self.varpi = (self.arg + self.node).wrap_at(2 * np.pi * u.rad)

        if kwargs.get('name') is not None:
            self.name = kwargs.get('name')
        else:
            # produces random, non-repeting integers between 0 and 1e10 - 1
            self.name = ['{:010}'.format(value) for value in random.sample(range(int(1e10)), len(self.a))]

        self.H = kwargs.get('h')
        if self.H is not None:
            self.mag = self.estimate_mag()
        if NSIDE is not None:
            for value in self.NSIDE:
                setattr(SpaceRock,
                        'HPIX_{}'.format(value),
                        self.radec_to_hpix(value))

        self.tau = Time(self.tau, format='jd', scale='utc')


    def calc_E(self, e, M):
        '''
        This method employs Newton's method to solve Kepler's Equation.
        '''
        f = lambda E, M, e: E - e * np.sin(E) - M
        E0 = M
        E = newton(f, E0, args=(M, e))
        return E


    @jit
    def calc_E_fast(self):
        E = self.M
        for kk in range(100):
            E = self.M + self.e * np.sin(E) * u.rad
        return E


    def xyz_to_equa(self):
        '''
        Transform from barycentric Cartesian coordinates to equatorial
        coordinates. If you need very precise values, the method will use
        a topocentric correction to the Earth's position.
        See https://en.wikipedia.org/wiki/Horizontal_coordinate_system.
        This results in a significant slowdown to the code.
        '''

        t = ts.tt(jd=self.tau.value)
        earth = planets['earth']

        # Only used for the topocentric calculation.
        if self.precise == True:
            if self.obscode is not None:
                earth += Topos(latitude_degrees=self.obslat,
                               longitude_degrees=self.obslon,
                               elevation_m=self.obselev) # topocentric calculation

        x_earth, y_earth, z_earth = earth.at(t).position.au * u.au # earth ICRS position
        earth_dis = norm([x_earth, y_earth, z_earth])
        x0, y0, z0 = self.x, self.y, self.z
        for idx in range(10):
            # transfer ecliptic to ICRS and shift to Geocentric (topocentric)
            x = x0 - x_earth
            y = y0 * np.cos(epsilon) - z0 * np.sin(epsilon) - y_earth
            z = y0 * np.sin(epsilon) + z0 * np.cos(epsilon) - z_earth
            delta = norm([x, y, z])
            ltt = delta / c
            M = self.M - ltt * (mu_bary / self.a**3)**0.5
            x, y, z = self.kep_to_xyz_pos(self.a, self.e, self.inc,
                                          self.arg, self.node, M, self.precise)


        # Cartesian to spherical coordinate
        self.delta = norm([x, y, z])
        self.ltt = self.delta / c
        self.dec = Angle(np.arcsin(z / norm([x, y, z])), u.rad)
        self.ra = Angle(np.arctan2(y, x), u.rad).wrap_at(2 * np.pi * u.rad)
        self.phase_angle = Angle(np.arccos(-(earth_dis**2 - self.r**2 - self.delta**2)/(2 * self.r* self.delta)), u.rad)
        self.elong = Angle(np.arccos(-(self.r**2 - self.delta**2 - earth_dis**2)/(2 * self.delta * earth_dis)), u.rad)
        self.skycoord = SkyCoord(self.ra, self.dec, frame='icrs')

        return self

    def sky_error(self):

        kep_to_xyz_jac = kep_to_xyz_jacobian(self.x.value,
                                             self.y.value,
                                             self.z.value,
                                             self.vx.value,
                                             self.vy.value,
                                             self.vz.value,
                                             mu_bary.value)
        J = np.linalg.inv(kep_to_xyz_jac)
        cov_xyz = np.matmul(J, np.matmul(self.cov_kep, J.T))


        return self


    def radec_to_hpix(self, NSIDE):
        '''
        Convert (ra, dec) into healpix.
        '''
        return hp.pixelfunc.ang2pix(NSIDE, np.pi/2 - self.dec.radian, self.ra.radian, nest=True)


    def estimate_mag(self):
        '''
        Estimate the apparent magnitude of a TNO
        '''
        q = (self.r**2 + self.delta**2 - 1 * u.au**2)/(2 * self.r * self.delta)

        ## pyephem
        beta = np.zeros(len(q))
        beta[np.where(q <= 1)[0]] = np.pi * u.rad
        beta[np.where(q >= 1)[0]] = 0 * u.rad

        Psi_1 = np.exp(-3.33 * np.tan(beta/2)**0.63)
        Psi_2 = np.exp(-1.87 * np.tan(beta/2)**1.22)
        mag = self.H + 5 * np.log10(self.r * self.delta / u.au**2)

        not_zero = np.where((Psi_1 != 0) | (Psi_2 != 0))[0]
        mag[not_zero] -= 2.5 * np.log10((1 - G) * Psi_1[not_zero] + G * Psi_2[not_zero])

        return mag


    def kep_to_xyz(self, mu):
        '''
        Transform from Keplerian to cartesian coordinates. There is no analytic
        solution to solve Kepler's Equation M = E - eSin[e] for the eccentric
        anomaly, E. If you need very precise coordinates, the method uses Newton's
        root-finding method from scipy.optimize. This is very precise, but it is
        not vectorizable or compilable, so it is rather slow
        (> factor of 5 slowdown). Otherwise, the .ethod uses a fixed-point
        iteration method. See https://en.wikipedia.org/wiki/Kepler%27s_equation
        '''
        # compute eccentric anomaly E
        if self.precise == True:
            E = np.array(list(map(self.calc_E, self.e.value, self.M.value))) * u.rad
        else:
            E = self.calc_E_fast()

        # compute true anomaly v
        ν = 2 * np.arctan2((1 + self.e)**0.5*np.sin(E/2.), (1 - self.e)**0.5*np.cos(E/2.))

        # compute the distance to the central body r
        r = self.a * (1 - self.e*np.cos(E))

        # obtain the position o and velocity ov vector
        o = [r * np.cos(ν), r * np.sin(ν), np.zeros(len(ν))]
        ov = [(mu * self.a)**0.5 / r * (-np.sin(E)),
              (mu * self.a)**0.5 / r * ((1-self.e**2)**0.5 * np.cos(E)),
              np.zeros(len(ν))]

        # Rotate o and ov to the inertial frame
        self.x, self.y, self.z = euler_rotation(self.arg, self.inc, self.node, o) * u.au
        self.vx, self.vy, self.vz = euler_rotation(self.arg, self.inc, self.node, ov) * u.au / u.day
        self.r = norm([self.x, self.y, self.z])
        return self


    def xyz_to_kep(self, mu):
        '''
        Transform from Cartesian to Keplerian coordinates. The units in this method
        are a bit hacky due to unexpected behavior and known issues with astropy
        units. But it returns the correct values with the correct units.
        '''
        ## there are a lot of hacky units in here because astropy doesn't behave as expected.
        # compute the barycentric distance r
        self.r = norm([self.x, self.y, self.z])
        rrdot = dot([self.x, self.y, self.z], [self.vx, self.vy, self.vz])

        # compute the specific angular momentum h
        hx, hy, hz = cross([self.x, self.y, self.z], [self.vx, self.vy, self.vz])
        h = norm([hx, hy, hz])

        # compute eccentricity vector
        ### hacky units
        ex, ey, ez = u.au**3 * u.rad**2/ u.day**2 * np.array(cross([self.vx, self.vy, self.vz], \
                     [hx, hy, hz])) / mu  - [self.x, self.y, self.z]*u.au/self.r
        self.e = norm([ex, ey, ez])

        # compute vector n
        ### hacky units
        nx, ny, nz = -hy.value, hx.value, np.zeros(len(hz))
        n = norm([nx, ny, nz])

        # compute true anomaly ν, the angle between e and r
        ν = np.arccos(dot([ex, ey, ez], [self.x, self.y, self.z]) / (self.e*self.r))
        ν[rrdot < 0] = 2 * np.pi * u.rad - ν[rrdot < 0]

        # compute inclination
        self.inc = Angle(np.arccos(hz/h), u.rad)

        # compute eccentric anomaly E
        E = 2 * np.arctan2(np.sqrt(1-self.e) * np.sin(ν/2), np.sqrt(1+self.e) * np.cos(ν/2))

        # compute ascending node
        node = np.arccos(nx/n)
        node[ny < 0] = 2 * np.pi - node[ny < 0]
        self.node = Angle(node, u.rad)

        # compute argument of periapsis, the angle between e and n
        arg = np.arccos(dot([nx, ny, nz], [ex, ey, ez]) / (n*self.e))
        arg[ez < 0] = 2 * np.pi * u.rad - arg[ez < 0]
        self.arg = Angle(arg, u.rad)

        # compute mean anomaly
        M = E - self.e * np.sin(E) * u.rad
        M[M < 0] += 2 * np.pi * u.rad
        self.M = Angle(M, u.rad)

        # compute a
        self.a = 1 / (2 / self.r - norm([self.vx, self.vy, self.vz])**2 / mu * u.rad**2)

        return self


    def to_bary(self):
        '''
        Method to convert heliocentric coordinates to barycentric coordinates.
        '''
        if self.frame == 'heliocentric':
            t = ts.tt(jd=self.tau.value)
            x_sun, y_sun, z_sun = sun.at(t).ecliptic_xyz().au * u.au
            vx_sun, vy_sun, vz_sun = sun.at(t).ecliptic_velocity().au_per_d * u.au / u.day
            # calculate the barycentric xyz postion
            self.x += x_sun
            self.y += y_sun
            self.z += z_sun
            self.vx += vx_sun
            self.vy += vy_sun
            self.vz += vz_sun

            # calculate barycentric keplerian elements
            self.xyz_to_kep(mu_bary)
            self.varpi = (self.arg + self.node).wrap_at(2 * np.pi * u.rad)
            lp = self.M < np.pi * u.rad
            self.epoch.jd[lp] = self.tau.value[lp] - self.M.value[lp] / np.sqrt(mu_bary.value / self.a.value[lp]**3)
            self.epoch.jd[~lp] = self.tau.value[~lp] + (2*np.pi - self.M.value[~lp]) / np.sqrt(mu_bary.value / self.a.value[~lp]**3)
            self.epoch = Time(self.epoch, format='jd', scale='utc')
            self.frame = 'barycentric'


    def to_helio(self):
        '''
        Method to convert barycentric coordinates to heliocentric coordinates.
        '''
        if self.frame == 'barycentric':
            t = ts.tt(jd=self.tau.value)
            x_sun, y_sun, z_sun = sun.at(t).ecliptic_xyz().au * u.au
            vx_sun, vy_sun, vz_sun = sun.at(t).ecliptic_velocity().au_per_d * u.au / u.day
            # calculate the heliocentric xyz postion
            self.x -= x_sun
            self.y -= y_sun
            self.z -= z_sun
            self.vx -= vx_sun
            self.vy -= vy_sun
            self.vz -= vz_sun

            # calculate heliocentric keplerian elements
            self.xyz_to_kep(mu_helio)

            self.varpi = (self.arg + self.node).wrap_at(2 * np.pi * u.rad)
            lp = self.M < np.pi * u.rad
            self.epoch.jd[lp] = self.tau.value[lp] - self.M.value[lp] / np.sqrt(mu_bary.value / self.a.value[lp]**3)
            self.epoch.jd[~lp] = self.tau.value[~lp] + (2*np.pi - self.M.value[~lp]) / np.sqrt(mu_bary.value / self.a.value[~lp]**3)
            self.epoch = Time(self.epoch, format='jd', scale='utc')
            self.frame = 'heliocentric'

        return self


    def kep_to_xyz_pos(self, a, e, inc, arg, node, M, precision):
        '''
        Just compute the xyz position of an object. Used for iterative equatorial
        calculation.
        '''
        # compute eccentric anomaly E
        if precision == True:
            E = np.array(list(map(self.calc_E, e.value, M.value))) * u.rad
        else:
            E = self.calc_E_fast()

        # compute true anomaly ν
        ν = 2 * np.arctan2((1 + e)**0.5*np.sin(E/2.), (1 - e)**0.5*np.cos(E/2.))

        # compute the distance to the central body r
        r = a * (1 - e * np.cos(E))

        # obtain the position vector o
        o = [r * np.cos(ν), r * np.sin(ν), np.zeros(len(ν))]

        # Rotate o to the inertial frame
        x, y, z = euler_rotation(arg, inc, node, o) * u.au

        return x, y, z


    def get_dict(self):
        '''
        Create a dictionary of the object attributes. This method is used
        by the pandas_df and astropy_table methods.
        '''
        data = {'name':self.name,
                'a':self.a,
                'e':self.e,
                'inc':self.inc,
                'arg':self.arg,
                'node':self.node,
                'varpi':self.varpi,
                'epoch':self.epoch,
                'M':self.M,
                'tau':self.tau,
                'x':self.x,
                'y':self.y,
                'z':self.z,
                'vx':self.vx,
                'vy':self.vy,
                'vz':self.vz,
                'ra':self.ra,
                'dec':self.dec,
                'skycoord':self.skycoord,
                'delta':self.delta,
                'ltt':self.ltt,
                'phase_angle':self.phase_angle,
                'elong':self.elong,
                'r':self.r}

        if self.NSIDE is not None:
            for value in self.NSIDE:
                data['HPIX_{}'.format(value)] = getattr(SpaceRock, 'HPIX_{}'.format(value))

        if self.H is not None:
            data['H'] = self.H
            data['mag'] = self.mag

        return data


    def pandas_df(self):
        '''
        Write the rocks to a pandas dataframe. Pandas can't handle astropy
        units (yet), so if you want to keep units intact you'll have to use
        an Astropy Table.
        '''
        return pd.DataFrame(self.get_dict())


    def astropy_table(self):
        '''
        Write the rocks to an astropy table. This can handle units, though
        it is generally less elegant than pandas.
        '''
        return Table(self.get_dict())


    def write_to_csv(self, path):
        '''
        Write the data to a csv.
        '''
        df = self.pandas_df()
        df.to_csv(path)
        return 'Data written to {}.csv.'.format(path)


    def plot_radec(self, color='black', alpha=0.5, zoom=False,
                   galactic_plane=False, ecliptic_plane=True):
        '''
        Plot the right ascension and declination of each object on a Mollweide
        projection. (See https://en.wikipedia.org/wiki/Mollweide_projection)
        '''
        if zoom == True:

            fig = plt.figure(figsize=(12, 8))
            ax = fig.add_subplot(111, projection=ccrs.PlateCarree())

            xdata = self.ra.degree
            xdata[xdata > 180] -= 360

            xmin=np.min(xdata)
            xmax=np.max(xdata)
            ymin=np.min(self.dec.degree)
            ymax=np.max(self.dec.degree)

            ax.scatter(-xdata, self.dec.degree, color='black', alpha=0.5)

            xticks = np.linspace(-xmax, -xmin, 8)
            yticks = np.linspace(ymin, ymax, 8)

            ax.set_xticks(xticks)
            ax.set_yticks(yticks)
            xticklabels = ['${:.2f}\degree$'.format(-value) for value in xticks]
            yticklabels = ['${:.2f}\degree$'.format(value) for value in yticks]
            ax.set_xticklabels(xticklabels)
            ax.set_yticklabels(yticklabels)

            gl = ax.gridlines(crs=ccrs.PlateCarree(), draw_labels=True,
                              linewidth=1, color='gray', alpha=0.5, linestyle='-')
            gl.xlocator = mticker.FixedLocator(xticks)
            gl.ylocator = mticker.FixedLocator(yticks)


            gl.bottom_labels = False
            gl.top_labels = False
            gl.left_labels = False
            gl.right_labels = False

            xrange = xmax - xmin
            yrange = ymax - ymin

            try:
                ax.set_extent([-xmax - xrange * 0.05, -xmin + xrange * 0.05,
                               ymin - yrange * 0.05, ymax + yrange * 0.05], crs=ccrs.PlateCarree())
            except:
                ax.set_extent([-180, 180, -90, 90], crs=ccrs.PlateCarree())

            if ecliptic_plane == True:
                def radec2project(ra, dec):
                    ra[ra>180] -= 360
                    return (ra, dec)

                plane_lon = np.linspace(-np.pi, np.pi, 1000)
                plane_lat = np.zeros(len(plane_lon))
                ecl_plane = np.zeros([len(plane_lat), 2])

                for i in range(len(plane_lat)):
                    ecl_plane[i] = ephem.Equatorial(ephem.Ecliptic(plane_lon[i], plane_lat[i])).get()

                x, y = radec2project(np.degrees(ecl_plane.T[0]), np.degrees(ecl_plane.T[1]))
                ax.plot(x[1:999], -y[1:999], 'r-', zorder=2)

            if galactic_plane == True:

                galactic = SkyCoord(l=np.linspace(0, 360, 1000)*u.degree, b=np.zeros(1000)*u.degree, frame='galactic')
                galxdata = galactic.icrs.ra.degree
                galxdata[galxdata > 180] -= 360
                galydata = galactic.icrs.dec.degree
                order = galxdata.argsort()
                ax.plot(-galxdata[order][1:999], -galydata[order][1:999], 'g-', zorder=3)

        else:

            fig = plt.figure(figsize=(12, 8))
            ax = fig.add_subplot(111, projection="mollweide")
            ax.grid(True)

            xdata = self.ra
            xdata[xdata.value > np.pi] -= 2*np.pi * u.rad
            ax.set_xticklabels(['$150\degree$', '$120\degree$', '$90\degree$' ,
                                '$60\degree$' , '$30\degree$' , '$0\degree$'  ,
                                '$330\degree$', '$300\degree$', '$270\degree$',
                                '$240\degree$', '$210\degree$'])

            ax.scatter(-xdata, self.dec, color=color, alpha=alpha)

            if ecliptic_plane == True:
                def radec2project(ra, dec):
                    ra[ra>180] -= 360
                    return (ra, dec)

                plane_lon = np.linspace(-np.pi, np.pi, 1000)
                plane_lat = np.zeros(len(plane_lon))
                ecl_plane = np.zeros([len(plane_lat), 2])

                for i in range(len(plane_lat)):
                    ecl_plane[i] = ephem.Equatorial(ephem.Ecliptic(plane_lon[i], plane_lat[i])).get()

                x, y = radec2project(np.degrees(ecl_plane.T[0]), np.degrees(ecl_plane.T[1]))
                ax.plot(np.radians(x[1:999]), -np.radians(y[1:999]), 'r-', zorder=2)

            if galactic_plane == True:

                galactic = SkyCoord(l=np.linspace(0, 360, 1000)*u.degree, b=np.zeros(1000)*u.degree, frame='galactic')
                galxdata = galactic.icrs.ra.rad
                galxdata[galxdata > np.pi] -= 2*np.pi
                galydata = galactic.icrs.dec.rad
                order = galxdata.argsort()
                ax.plot(-galxdata[order][1:999], galydata[order][1:999], 'g-', zorder=3)

        return fig, ax


    def set_simulation(self, startdate, model, gr=False, gr_fast=False,
                       gr_full=False, add_pluto=False):

        mercury = planets['mercury']
        venus = planets['venus']
        earth = planets['earth']
        mars = planets['mars']
        jupiter = planets['jupiter barycenter']
        saturn = planets['saturn barycenter']
        uranus = planets['uranus barycenter']
        neptune = planets['neptune barycenter']

        M_mercury = 1.6601367952719304e-7
        M_venus = 2.4478383396645447e-6
        M_earth = 3.040432648022642e-6 # Earth-Moon Barycenter
        M_mars = 3.2271560375549977e-7 # Mars Barycenter
        M_sun = 1
        M_jupiter = 9.547919384243222e-4
        M_saturn = 2.858859806661029e-4
        M_uranus = 4.3662440433515637e-5
        M_neptune = 5.151389020535497e-5
        M_pluto = 7.361781606089469e-9

        if model == 0:
            active_bodies = [sun]
            names = ['Sun']
            M_sun += M_mercury + M_venus + M_earth + M_mars \
                     + M_jupiter + M_saturn + M_uranus + M_neptune
            masses = [M_sun]

        elif model == 1:
            active_bodies = [sun, jupiter, saturn, uranus, neptune]
            names = ['Sun', 'Jupiter', 'Saturn', 'Uranus', 'Neptune']
            M_sun += M_mercury + M_venus + M_earth + M_mars
            masses = [M_sun, M_jupiter, M_saturn, M_uranus, M_neptune]

        elif model == 2:
            active_bodies = [sun, earth, jupiter, saturn, uranus, neptune]
            names = ['Sun', 'Earth', 'Jupiter', 'Saturn', 'Uranus', 'Neptune']
            M_sun += M_mercury + M_venus + M_mars
            masses = [M_sun, M_earth, M_jupiter, M_saturn, M_uranus, M_neptune]

        elif model == 3:
            active_bodies = [sun, earth, mars, jupiter, saturn, uranus, neptune]
            names = ['Sun', 'Earth', 'Mars', 'Jupiter', 'Saturn', 'Uranus', 'Neptune']
            M_sun += M_mercury + M_venus
            masses = [M_sun, M_earth, M_Mars,
                      M_jupiter, M_saturn, M_uranus, M_neptune]

        elif model == 4:
            active_bodies = [sun, venus, earth, mars, jupiter, saturn, uranus, neptune]
            names = ['Sun', 'Venus', 'Earth', 'Mars', 'Jupiter', 'Saturn', 'Uranus', 'Neptune']
            M_sun += M_mercury
            masses = [M_sun, M_venus, M_earth, M_Mars,
                      M_jupiter, M_saturn, M_uranus, M_neptune]

        elif model == 5:
            active_bodies = [sun, mercury, venus, earth, mars, jupiter, saturn, uranus, neptune]
            names = ['Sun', 'Mercury', 'Venus', 'Earth', 'Mars', 'Jupiter', 'Saturn', 'Uranus', 'Neptune']
            masses = [M_sun, M_mercury, M_venus, M_earth, M_Mars, M_jupiter, M_saturn, M_uranus, M_neptune]

        if add_pluto == True:
            pluto = planets['pluto barycenter']
            active_bodies.append(pluto)
            names.append('Pluto')
            masses.append(M_pluto)

        t = ts.tt(jd=startdate)

        x, y, z = np.array([body.at(t).ecliptic_xyz().au for body in active_bodies]).T
        vx, vy, vz = np.array([body.at(t).ecliptic_velocity().au_per_d for body in active_bodies]).T

        # create a dataframe of the massive bodies in the solar system
        ss = pd.DataFrame()
        ss['x'] = x
        ss['y'] = y
        ss['z'] = z
        ss['vx'] = vx
        ss['vy'] = vy
        ss['vz'] = vz
        ss['mass'] = masses
        ss['a'] = 1 / (2 / norm([ss.x, ss.y, ss.z]) - norm([ss.vx, ss.vy, ss.vz])**2 / mu_bary.value)
        ss['hill_radius'] = ss.a * pow(ss.mass / (3 * M_sun), 1/3)
        ss['name'] = names

        sim = rebound.Simulation()
        sim.units = ('day', 'AU', 'Msun')

        for p in ss.itertuples():
            sim.add(x=p.x, y=p.y, z=p.z,
                    vx=p.vx, vy=p.vy, vz=p.vz,
                    m=p.mass, hash=p.name, r=p.hill_radius)

        sim.move_to_com()

        if gr_fast == True:
            bodies = sim.particles
            rebx = reboundx.Extras(sim)
            gr = rebx.load_force('gr_potential')
            gr.params['c'] = constants.C
            rebx.add_force(gr)
            bodies['Sun'].params['gr_source'] = 1

        elif gr == True:
            bodies = sim.particles
            rebx = reboundx.Extras(sim)
            gr = rebx.load_force('gr')
            gr.params['c'] = constants.C
            rebx.add_force(gr)
            bodies['Sun'].params['gr_source'] = 1

        elif gr_full == True:
            rebx = reboundx.Extras(sim)
            gr = rebx.load_force('gr_full')
            gr.params["c"] = constants.C
            rebx.add_force(gr)

        sim.N_active = len(ss)
        sim.testparticle_type = 0

        sim.integrator = 'mercurius'
        sim.dt = 1 # one day

        sim.ri_ias15.min_dt = sim.dt / 1440 # one minute
        sim.ri_mercurius.hillfac = 3

        return sim


    def propagate(self, enddates, model=0):
        '''
        Integrate the bodies to the desired date. The logic could be cleaner
        but it works.
        '''

        if np.isscalar(enddates):
            enddates = np.array([enddates])

        Nx = len(enddates)
        Ny = len(self.x)
        x_values = np.zeros([Nx, Ny])
        y_values = np.zeros([Nx, Ny])
        z_values = np.zeros([Nx, Ny])
        vx_values = np.zeros([Nx, Ny])
        vy_values = np.zeros([Nx, Ny])
        vz_values = np.zeros([Nx, Ny])
        tau_values = np.zeros([Nx, Ny])
        name_values = np.tile(self.name, Nx)
        if self.H is not None:
            H_values = np.tile(self.H, Nx)

        in_frame = self.frame

        # We need to (or should) work in barycentric coordinates in rebound
        if in_frame == 'heliocentric':
            self.to_bary()

        # Rehash as a dataframe for easy access
        df = self.pandas_df()
        df['tau'] = df['tau'].apply(lambda idx: idx.jd)

        # Integrate all particles to the same tau
        pickup_times = df.tau
        sim = self.set_simulation(np.min(pickup_times), model)
        sim.t = np.min(df.tau)

        for time in np.sort(np.unique(pickup_times)):
            ps = df[df.tau == time]
            for p in ps.itertuples():
                sim.add(x=p.x, y=p.y, z=p.z,
                vx=p.vx, vy=p.vy, vz=p.vz,
                hash=p.name)
                sim.integrate(time, exact_finish_time=1)

        for ii, time in enumerate(np.sort(enddates)):
            sim.integrate(time, exact_finish_time=1)
            for jj, name in enumerate(self.name):
                x_values[ii, jj] = sim.particles[name].x
                y_values[ii, jj] = sim.particles[name].y
                z_values[ii, jj] = sim.particles[name].z
                vx_values[ii, jj] = sim.particles[name].vx
                vy_values[ii, jj] = sim.particles[name].vy
                vz_values[ii, jj] = sim.particles[name].vz
                tau_values[ii, jj] = sim.t

        self.x = x_values.flatten() * u.au
        self.y = y_values.flatten() * u.au
        self.z = z_values.flatten() * u.au
        self.vx = vx_values.flatten() * (u.au / u.day)
        self.vy = vy_values.flatten() * (u.au / u.day)
        self.vz = vz_values.flatten() * (u.au / u.day)
        self.name = name_values.flatten()
        self.tau = Time(tau_values.flatten(), format='jd', scale='utc')

        self.xyz_to_kep(mu_bary)

        self.epoch = np.zeros(Nx * Ny)
        lp = self.M < np.pi * u.rad
        self.epoch[lp] = self.tau.jd[lp] * u.day - self.M[lp] / np.sqrt(mu_bary / self.a[lp]**3)
        self.epoch[~lp] = self.tau.jd[~lp] * u.day + (2*np.pi * u.rad - self.M[~lp]) / np.sqrt(mu_bary / self.a[~lp]**3)
        self.epoch = Time(self.epoch, format='jd', scale='utc')

        self.xyz_to_equa()
        self.varpi = (self.arg + self.node).wrap_at(2 * np.pi * u.rad)

        if self.H is not None:
            self.H = H_values
            self.mag = self.estimate_mag()

        # calculate new hpix
        if self.NSIDE is not None:
            for value in self.NSIDE:
                setattr(SpaceRock,
                'HPIX_{}'.format(value),
                self.radec_to_hpix(value))

        # be polite and return orbital parameters in the input frame.
        if in_frame == 'heliocentric':
            self.to_helio()

        out_df = self.pandas_df()

        return df, out_df
