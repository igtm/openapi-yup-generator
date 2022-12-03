/* tslint:disable */
/* eslint-disable */
//DO NOT EDIT MANUALLY

import { object, string, number, date, array, bool } from 'yup';

export const LightingSummary = object({
  zones: array().of(object({
    id: string(),
    name: string(),
    deviceId: number().integer(),
    deviceType: string(),
    zone: string(),
  })),
  zoneStatus: array().of(object({
    id: string(),
    name: string(),
    lastUpdate: date(),
    level: number().integer(),
  }).label('the status of the lighting zone.')),
}).label('ok');

export const LightingZone = object({
  id: string(),
  name: string(),
  deviceId: number().integer(),
  deviceType: string(),
  zone: string(),
});

export const LightingZoneStatus = object({
  id: string(),
  name: string(),
  lastUpdate: date(),
  level: number().integer(),
}).label('the status of the lighting zone.');

export const TemperatureSummary = object({
  zones: array().of(object({
    id: number().integer().label('the unique identifier for the zone').required(),
    name: string().required(),
    inputPosition: number().integer(),
    outputPosition: number().integer(),
    zone: string(),
  }).label('a single temperature zone')),
  zoneStatus: array().of(object({
    id: string().label('the unique identifier for the zone').required(),
    name: string().label('the name of the zone'),
    value: number().label('the temperature in the zone').required(),
    units: string().label('the temperature units'),
    timestamp: date().label('the timestamp when the temperature was measured').required(),
  }).label('status of a single zone')),
}).label('ok');

export const TemperatureZone = object({
  id: number().integer().label('the unique identifier for the zone').required(),
  name: string().required(),
  inputPosition: number().integer(),
  outputPosition: number().integer(),
  zone: string(),
}).label('a single temperature zone');

export const TemperatueZoneStatus = object({
  id: string().label('the unique identifier for the zone').required(),
  name: string().label('the name of the zone'),
  value: number().label('the temperature in the zone').required(),
  units: string().label('the temperature units'),
  timestamp: date().label('the timestamp when the temperature was measured').required(),
}).label('status of a single zone');

export const ApiResponse = object({
  code: number().integer(),
  message: string(),
});

export const HeaterState = object({
  id: string(),
  state: string(),
});

export const DeviceState = object({
  id: string(),
  name: string(),
  lastUpdate: date(),
  level: number().integer(),
});

export const ForecastResponse = object({
  city: object({
    id: number().integer(),
    name: string(),
    lat: number(),
    lon: number(),
    country: string(),
  }),
  values: array().of(object({
    date: date(),
    pressure: number(),
    humidity: number().integer(),
    windSpeed: number(),
    clouds: number().integer(),
    temperature: object({
      low: number(),
      high: number(),
      morning: number(),
      day: number(),
      evening: number(),
      night: number(),
    }),
    weather: object({
      summary: string(),
      description: string(),
      icon: string(),
    }),
  })),
});

export const Forecast = object({
  date: date(),
  pressure: number(),
  humidity: number().integer(),
  windSpeed: number(),
  clouds: number().integer(),
  temperature: object({
    low: number(),
    high: number(),
    morning: number(),
    day: number(),
    evening: number(),
    night: number(),
  }),
  weather: object({
    summary: string(),
    description: string(),
    icon: string(),
  }),
});

export const City = object({
  id: number().integer(),
  name: string(),
  lat: number(),
  lon: number(),
  country: string(),
});

export const ForecastTemperature = object({
  low: number(),
  high: number(),
  morning: number(),
  day: number(),
  evening: number(),
  night: number(),
});

export const WeatherForecast = object({
  summary: string(),
  description: string(),
  icon: string(),
});

export const DeviceRegistrationInfo = object({
  uri: string(),
  id: string(),
});

