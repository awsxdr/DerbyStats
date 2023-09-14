import React, { useEffect } from "react";
import { useSearchParams } from "react-router-dom";

const getSocket = () => new WebSocket(`ws://${location.hostname}:8001/ws`);
//const getSocket() => new WebSocket(`ws://${location.hostname}:${location.port}/ws`);

const subscribeToState = <TState>(stateType: string, gameId: string, onUpdate: (state: TState) => void) => {
    const socket = getSocket();

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

    return () => {
        socket.close();
    };

}

export const useGlobalStateSocket = <TState>(stateType: string, onUpdate: (state: TState) => void, dependencies?: React.DependencyList) => {
    useEffect(() => {
        subscribeToState(stateType, "*", onUpdate);
    }, [...(dependencies || [])]);
}

export const useStateSocket = <TState>(stateType: string, onUpdate: (state: TState) => void, dependencies?: React.DependencyList) => {
    //const socket = useMemo(() => new WebSocket(`ws://${location.hostname}:${location.port}/ws`), []);

    const [searchParams] = useSearchParams();

    useEffect(() => {
        const gameId = searchParams.get('gameId');

        if(!gameId) {
            return;
        }

        subscribeToState(stateType, gameId, onUpdate);
    
    }, [...(dependencies || []), searchParams])
};