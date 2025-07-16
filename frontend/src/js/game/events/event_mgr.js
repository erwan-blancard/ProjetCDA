import { GameEvent } from "./events";
import * as GAME from "../game";


// class used to store and execute events
export class EventMgr {
    /** @type {Array<GameEvent>} */
    #queue = [];
    #waiting_for_events = false;

    constructor() {}

    pushEvent(event) {
        event.mgr = this;
        this.#queue.push(event);
        if (!this.#waiting_for_events) {
            this.executeNext();
        }
    }

    pushEvents(events) {
        events.forEach(event => {
            event.mgr = this;
            this.#queue.push(event);
        });
        if (!this.#waiting_for_events) {
            this.executeNext();
        }
    }

    emptyQueue() { this.#queue.length = 0; }
    isWaitingForEvents() { return this.#waiting_for_events; }
    get queueCount() { return this.#queue.length; }

    executeNext() {
        if (this.#queue.length > 0) {
            // remove event and execute it
            const event = this.#queue.splice(0, 1)[0];
            this.#waiting_for_events = true;
            event.execute();
        } else {
            this.#waiting_for_events = false;
        }
    }

}
