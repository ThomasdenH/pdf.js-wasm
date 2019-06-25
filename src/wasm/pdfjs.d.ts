/* tslint:disable */
/**
* @param {number} level 
* @returns {void} 
*/
export function setVerbosityLevel(level: number): void;
/**
* @returns {number} 
*/
export function getVerbosityLevel(): number;
/**
* @param {number} number 
* @param {boolean | undefined} lowercase 
* @returns {string} 
*/
export function toRomanNumerals(number: number, lowercase?: boolean): string;
/**
* @param {Uint8Array} bytes 
* @returns {number} 
*/
export function computeAdler32(bytes: Uint8Array): number;
