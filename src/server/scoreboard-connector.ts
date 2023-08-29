import WebSocket from 'ws';

type StateValue = number | string;
export type State = Map<string, StateValue>;
interface StateUpdate {
    state: State;
}

const combineMaps = <K, V>(map1: Map<K, V>, map2: Map<K, V>) => new Map<K, V>([
    ...Array.from(map1.entries()),
    ...Array.from(map2.entries())
]);

type ConnectHandler = () => void;
type UpdateHandler = (state: State) => void;

type EventType = "Connect" | "Update";
type EventHandler = ConnectHandler | UpdateHandler;

type UpdateMatchPredicate = (updateType: string) => boolean;

type UpdateHandlerDetails = {
    predicate: UpdateMatchPredicate,
    handler: UpdateHandler,
};

export class ScoreboardConnector {
    #socket: WebSocket;
    #isOpen: boolean;
    #connectHandlers: ConnectHandler[] = [];
    #updateHandlers: UpdateHandlerDetails[] = [];

    state: State;

    constructor(scoreboardEndpoint: string) {
        this.#isOpen = false;

        this.#socket = new WebSocket(`ws://${scoreboardEndpoint}/WS/`);

        this.#socket.on('open', this.#handleOpen.bind(this));
        this.#socket.on('error', () => this.#isOpen = false);
        this.#socket.on('message', this.#handleMessage.bind(this));

        this.state = new Map<string, StateValue>();
    }

    listenForTopics(topics: string[]) {
        topics.forEach(topic => {
            console.log(`Registering topic '${topic}'`);
        });

        this.#socket.send(JSON.stringify({
            action: 'Register',
            paths: topics
        }));
    }

    addUpdateHandler(predicate: UpdateMatchPredicate, handler: UpdateHandler) {
        this.#updateHandlers.push({ predicate, handler });
    }

    isOpen() {
        return this.#isOpen;
    }

    on(_event: "Connect", handler: ConnectHandler): void;
    on(event: EventType, handler: EventHandler) {
        switch(event) {
            case "Connect":
                this.#connectHandlers.push(handler as ConnectHandler);
                break;
        }
    }

    #handleOpen() {
        console.log('Connection to scoreboard opened');

        this.#isOpen = true;
        this.#connectHandlers.forEach(h => h());
    }

    #handleMessage(data: Buffer) {
        const update: StateUpdate = JSON.parse(data.toString());
        const updateState = new Map<string, StateValue>(Object.entries(update.state));

        this.state = combineMaps(this.state, updateState);

        this.#updateHandlers.forEach(h => {
            if(!Object.keys(update.state).every(k => !h.predicate(k))) {
                h.handler(this.state);
            }
        });
    }
};