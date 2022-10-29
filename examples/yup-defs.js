/* tslint:disable */
/* eslint-disable */
//DO NOT EDIT MANUALLY

import { object, string, number, date, array, bool } from 'yup';

export const LightingSummary = object({
  zones: array().of(object({
    id: string().optional(),
    name: string().optional(),
    deviceId: number().integer().optional(),
    deviceType: string().optional(),
    zone: string().optional(),
  })).optional(),
  zoneStatus: array().of(object({
    id: string().optional(),
    name: string().optional(),
    lastUpdate: date().optional(),
    level: number().integer().optional(),
  }).label('the status of the lighting zone.')).optional(),
}).label('ok');

export const LightingZone = object({
  id: string().optional(),
  name: string().optional(),
  deviceId: number().integer().optional(),
  deviceType: string().optional(),
  zone: string().optional(),
});

export const LightingZoneStatus = object({
  id: string().optional(),
  name: string().optional(),
  lastUpdate: date().optional(),
  level: number().integer().optional(),
}).label('the status of the lighting zone.');

export const TemperatureSummary = object({
  zones: array().of(object({
    id: number().integer().label('the unique identifier for the zone').required(),
    name: string().required(),
    inputPosition: number().integer().optional(),
    outputPosition: number().integer().optional(),
    zone: string().optional(),
  }).label('a single temperature zone')).optional(),
  zoneStatus: array().of(object({
    id: string().label('the unique identifier for the zone').required(),
    name: string().label('the name of the zone').optional(),
    value: number().label('the temperature in the zone').required(),
    units: string().label('the temperature units').optional(),
    timestamp: date().label('the timestamp when the temperature was measured').required(),
  }).label('status of a single zone')).optional(),
}).label('ok');

export const TemperatureZone = object({
  id: number().integer().label('the unique identifier for the zone').required(),
  name: string().required(),
  inputPosition: number().integer().optional(),
  outputPosition: number().integer().optional(),
  zone: string().optional(),
}).label('a single temperature zone');

export const TemperatueZoneStatus = object({
  id: string().label('the unique identifier for the zone').required(),
  name: string().label('the name of the zone').optional(),
  value: number().label('the temperature in the zone').required(),
  units: string().label('the temperature units').optional(),
  timestamp: date().label('the timestamp when the temperature was measured').required(),
}).label('status of a single zone');

export const ApiResponse = object({
  code: number().integer().optional(),
  message: string().optional(),
});

export const HeaterState = object({
  id: string().optional(),
  state: string().optional(),
});

export const DeviceState = object({
  id: string().optional(),
  name: string().optional(),
  lastUpdate: date().optional(),
  level: number().integer().optional(),
});

export const ForecastResponse = object({
  city: object({
    id: number().integer().optional(),
    name: string().optional(),
    lat: number().optional(),
    lon: number().optional(),
    country: string().optional(),
  }).optional(),
  values: array().of(object({
    date: date().optional(),
    pressure: number().optional(),
    humidity: number().integer().optional(),
    windSpeed: number().optional(),
    clouds: number().integer().optional(),
    temperature: object({
      low: number().optional(),
      high: number().optional(),
      morning: number().optional(),
      day: number().optional(),
      evening: number().optional(),
      night: number().optional(),
    }).optional(),
    weather: object({
      summary: string().optional(),
      description: string().optional(),
      icon: string().optional(),
    }).optional(),
  })).optional(),
});

export const Forecast = object({
  date: date().optional(),
  pressure: number().optional(),
  humidity: number().integer().optional(),
  windSpeed: number().optional(),
  clouds: number().integer().optional(),
  temperature: object({
    low: number().optional(),
    high: number().optional(),
    morning: number().optional(),
    day: number().optional(),
    evening: number().optional(),
    night: number().optional(),
  }).optional(),
  weather: object({
    summary: string().optional(),
    description: string().optional(),
    icon: string().optional(),
  }).optional(),
});

export const City = object({
  id: number().integer().optional(),
  name: string().optional(),
  lat: number().optional(),
  lon: number().optional(),
  country: string().optional(),
});

export const ForecastTemperature = object({
  low: number().optional(),
  high: number().optional(),
  morning: number().optional(),
  day: number().optional(),
  evening: number().optional(),
  night: number().optional(),
});

export const WeatherForecast = object({
  summary: string().optional(),
  description: string().optional(),
  icon: string().optional(),
});

export const DeviceRegistrationInfo = object({
  uri: string().optional(),
  id: string().optional(),
});

