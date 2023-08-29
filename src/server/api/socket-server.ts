import { Application } from "express-ws";
import WebSocket from "ws";

interface ISocketMessage {
    messageType: string;
}

class SubscribeSocketMessage implements ISocketMessage {
    messageType: "Subscribe" = "Subscribe";
    dataType = "";
}

export interface IUpdateProvider {
    getState(): any;
}

export default class SocketServer {
    #updateHandlers: Map<string, WebSocket[]> = new Map();
    #updateProviders: Map<string, IUpdateProvider> = new Map();

    constructor(app: Application) {
        app.ws("/ws", (socket, _request) => {
            console.log("WebSocket connection received");

            socket.on('message', (data) => {
                console.log("Message received from client");
                const message: ISocketMessage = JSON.parse(data.toString());

                switch(message.messageType) {
                    case "Subscribe":
                        this.#handleSubscribe(message as SubscribeSocketMessage, socket);
                        break;

                    default:
                        console.log(`Received unexpected message type: ${message.messageType}`);
                        break;
                }
            });
        });
    }

    sendUpdate<TUpdate>(dataType: string, update: TUpdate) {
        console.log(`Sending update for '${dataType}'`);

        this.#updateHandlers.get(dataType)?.forEach(handler => {
            if(handler.readyState != handler.OPEN) {
                return;
            }

            handler.send(JSON.stringify({
                dataType,
                body: update
            }));
        });
    }

    setUpdateProvider(dataType: string, updateProvider: IUpdateProvider) {
        this.#updateProviders.set(dataType, updateProvider);
    }

    #handleSubscribe(message: SubscribeSocketMessage, socket: WebSocket) {
        if(!this.#updateHandlers.has(message.dataType)) {
            this.#updateHandlers.set(message.dataType, []);
        }

        this.#updateHandlers.get(message.dataType)?.push(socket);

        console.log(`New subscriber for '${message.dataType}'. Sending latest state.`);

        const currentState = this.#updateProviders.get(message.dataType)?.getState();
        socket.send(JSON.stringify({
            dataType: message.dataType,
            body: currentState,
        }));
    }
}