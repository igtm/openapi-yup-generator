/* tslint:disable */
/* eslint-disable */
//DO NOT EDIT MANUALLY

import { object, string, number, date, array, bool } from 'yup';

export const ApiResponse = object({
  code: number().integer().optional(),
  message: string().optional(),
});

export const City = object({
  country: string().optional(),
  id: number().integer().optional(),
  lat: number().optional(),
  lon: number().optional(),
  name: string().optional(),
});

export const DeviceRegistrationInfo = object({
  id: string().optional(),
  uri: string().optional(),
});

export const DeviceState = object({
  id: string().optional(),
  lastUpdate: string().optional(),
  level: number().integer().optional(),
  name: string().optional(),
});

export const Forecast = object({
  clouds: number().integer().optional(),
  date: string().optional(),
  humidity: number().integer().optional(),
  pressure: number().optional(),
  temperature: object().optional(),
  weather: object().optional(),
  windSpeed: number().optional(),
});

export const ForecastResponse = object({
  city: object().optional(),
  values: array().optional(),
});

export const ForecastTemperature = object({
  day: number().optional(),
  evening: number().optional(),
  high: number().optional(),
  low: number().optional(),
  morning: number().optional(),
  night: number().optional(),
});

export const HeaterState = object({
  id: string().optional(),
  state: string().optional(),
});

export const LightingSummary = object({
  zoneStatus: array().optional(),
  zones: array().optional(),
});

export const LightingZone = object({
  deviceId: number().integer().optional(),
  deviceType: string().optional(),
  id: string().optional(),
  name: string().optional(),
  zone: string().optional(),
});

export const LightingZoneStatus = object({
  id: string().optional(),
  lastUpdate: string().optional(),
  level: number().integer().optional(),
  name: string().optional(),
});

export const TemperatueZoneStatus = object({
  id: string().required(),
  name: string().optional(),
  timestamp: string().required(),
  units: string().optional(),
  value: number().required(),
});

export const TemperatureSummary = object({
  zoneStatus: array().optional(),
  zones: array().optional(),
});

export const TemperatureZone = object({
  id: number().integer().required(),
  inputPosition: number().integer().optional(),
  name: string().required(),
  outputPosition: number().integer().optional(),
  zone: string().optional(),
});

export const WeatherForecast = object({
  description: string().optional(),
  icon: string().optional(),
  summary: string().optional(),
});

