import WebSocket from 'ws';

type StateValue = number | string;
type State = Map<string, StateValue>;
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

export class ScoreboardConnector {
    #socket: WebSocket;
    #isOpen: boolean;
    #connectHandlers: ConnectHandler[] = [];
    #updateHandlers: UpdateHandler[] = [];

    state: State;

    constructor(scoreboardEndpoint: string) {
        this.#isOpen = false;

        this.#socket = new WebSocket(`ws://${scoreboardEndpoint}/WS/`);

        this.#socket.on('open', this.#handleOpen.bind(this));
        this.#socket.on('error', () => this.#isOpen = false);
        this.#socket.on('message', this.#handleMessage.bind(this));

        this.state = new Map<string, StateValue>();
    }

    listenForTopic(topic: string) {
        console.log(`Registering topic '${topic}'`);

        this.#socket.send(JSON.stringify({
            action: 'Register',
            paths: [ topic ]
        }));
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

        this.#updateHandlers.forEach(h => h(this.state));
    }

    isOpen() {
        return this.#isOpen;
    }

    on(_event: "Connect", handler: ConnectHandler): void;
    on(_event: "Update", handler: UpdateHandler): void;
    on(event: EventType, handler: EventHandler) {
        switch(event) {
            case "Connect":
                this.#connectHandlers.push(handler as ConnectHandler);
                break;

            case "Update":
                this.#updateHandlers.push(handler as UpdateHandler);
                break;
        }

    }
};