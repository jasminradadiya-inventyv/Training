// @ts-check

/**
 * Create an appointment
 *
 * @param {number} days
 * @param {number} [now] (ms since the epoch, or undefined)
 *
 * @returns {Date} the appointment
 */
export function createAppointment(days, now = undefined) {
  const timeNow = now ? new Date(now) : new Date;
  timeNow.setDate(timeNow.getDate()+days);
  return timeNow;
}

/**
 * Generate the appointment timestamp
 *
 * @param {Date} appointmentDate
 *
 * @returns {string} timestamp
 */
export function getAppointmentTimestamp(appointmentDate) {
  return appointmentDate.toISOString();
}

/**
 * Get details of an appointment
 *
 * @param {string} timestamp (ISO 8601)
 *
 * @returns {Record<'year' | 'month' | 'date' | 'hour' | 'minute', number>} the appointment details
 */
export function getAppointmentDetails(timestamp) {
  const record = new Date(timestamp);
  
  return { 
    year: record.getFullYear(), 
    month: record.getMonth(), 
    date: record.getDate(), 
    hour: record.getHours(), 
    minute: record.getMinutes()
  };
}


/**
 * Update an appointment with given options
 *
 * @param {string} timestamp (ISO 8601)
 * @param {Partial<Record<'year' | 'month' | 'date' | 'hour' | 'minute', number>>} options
 *
 * @returns {Record<'year' | 'month' | 'date' | 'hour' | 'minute', number>} the appointment details
 */
export function updateAppointment(timestamp, options) {
  
  let record = new Date(timestamp);
  console.log(options)
  if(options.year != null) record.setFullYear(options.year)
  if(options.month != null) record.setMonth(options.month)
  if(options.date != null) record.setDate(options.date)
  if(options.hour != null) record.setHours(options.hour)
  if(options.minute != null) record.setMinutes(options.minute)

  return getAppointmentDetails(record);
}

/**
 * Get available time in seconds (rounded) between two appointments
 *
 * @param {string} timestampA (ISO 8601)
 * @param {string} timestampB (ISO 8601)
 *
 * @returns {number} amount of seconds (rounded)
 */
export function timeBetween(timestampA, timestampB) {
  let tA = new Date(timestampA)
  let tB = new Date(timestampB)

  let diff = Math.abs(tB.getTime() - tA.getTime());
  diff = diff/1000;
  return Math.round(diff);
}

/**
 * Get available times between two appointment
 *
 * @param {string} appointmentTimestamp (ISO 8601)
 * @param {string} currentTimestamp (ISO 8601)
 */
export function isValid(appointmentTimestamp, currentTimestamp) {
  return new Date(appointmentTimestamp) > new Date(currentTimestamp)
}
