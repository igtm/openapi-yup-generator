/* tslint:disable */
/* eslint-disable */
//DO NOT EDIT MANUALLY

import { object, string, number, date, array, bool } from 'yup';

export const LightingSummary = object({
  zones: array().optional(),
  zoneStatus: array().optional(),
});

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
});

export const TemperatureSummary = object({
  zones: array().optional(),
  zoneStatus: array().optional(),
});

export const TemperatureZone = object({
  id: number().integer().required(),
  name: string().required(),
  inputPosition: number().integer().optional(),
  outputPosition: number().integer().optional(),
  zone: string().optional(),
});

export const TemperatueZoneStatus = object({
  id: string().required(),
  name: string().optional(),
  value: number().required(),
  units: string().optional(),
  timestamp: date().required(),
});

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
  values: array().optional(),
});

export const Forecast = object({
  date: date().optional(),
  pressure: number().optional(),
  humidity: number().integer().optional(),
  windSpeed: number().optional(),
  clouds: number().integer().optional(),
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

