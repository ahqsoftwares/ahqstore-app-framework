/**
 * Tries to connect to AHQ Store Framework Agent
 * Resolves if it succeeds, rejects if it fails
 * @returns {Promise<void>}
 */
export function start(): Promise<void>;
/**
 * Listen to WebSocket Events
 * @param {Function} fn
 * @returns {number} total number of listeners
 */
export function listenWs(fn: Function): number;
/**
 * Sends a message to the electron app
 * @param {{
 *  require: string,
 *  ref: string,
 *  data: any
 * }} data
 */
export function sendWsMessage(data: {
    require: string;
    ref: string;
    data: any;
}): void;
/**
 * Get the raw ws that is being wrapped
 * @returns {WebSocket}
 */
export function getRawWs(): WebSocket;
import { WebSocket } from "ws";
//# sourceMappingURL=package.d.ts.map