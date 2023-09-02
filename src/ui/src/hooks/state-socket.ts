import { useMemo } from "react";
import { useSearchParams } from "react-router-dom";

export const useStateSocket = <TState>(stateType: string, onUpdate: (state: TState) => void) => {
    const socket = useMemo(() => new WebSocket(`ws://${location.hostname}:8003/ws`), []);

    const [searchParams] = useSearchParams();

    const gameId = searchParams.get('gameId');

    socket.addEventListener('open', () => {
        socket.send(JSON.stringify({
            messageType: "Subscribe",
            gameId,
            dataType: stateType
        }));
    });
    
    socket.addEventListener('message', (event) => {
        onUpdate(JSON.parse(event.data));
    });
};