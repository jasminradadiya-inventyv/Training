// @ts-check

/**
 * Generates a random starship registry number.
 *
 * @returns {string} the generated registry number.
 */
export function randomShipRegistryNumber() {
  let num = Math.floor(Math.random()*1000 + 1000);
  console.log(num)
  return `NCC-${num}`
}

/**
 * Generates a random stardate.
 *
 * @returns {number} a stardate between 41000 (inclusive) and 42000 (exclusive).
 */
export function randomStardate() {
  let num = Math.random() * (1000) + 41000;
  return num;
}

/**
 * Generates a random planet class.
 *
 * @returns {string} a one-letter planet class.
 */
export function randomPlanetClass() {
  let rangeChar = ['D','H','J','K','L','M','N','R','T','Y'];
  let num = Math.floor(Math.random() * (10));
  return rangeChar[num];
}
