import React, { useEffect } from "react";
import { useSearchParams } from "react-router-dom";

export const useStateSocket = <TState>(stateType: string, onUpdate: (state: TState) => void, dependencies?: React.DependencyList) => {
    //const socket = useMemo(() => new WebSocket(`ws://${location.hostname}:${location.port}/ws`), []);

    const [searchParams] = useSearchParams();

    useEffect(() => {
        console.log('Mount');

        const socket = new WebSocket(`ws://${location.hostname}:8001/ws`);

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

        return () => {
            console.log('Unmount');
            socket.close();
        };
    }, [...(dependencies || [])])
};