import { Label, LineChart, Line, XAxis, YAxis, Tooltip } from 'recharts';

import { useDarkThemeContext } from '../../../contexts/';
import { GraphContainer } from '../../atoms/GraphContainer/GraphContainer';
import { useMemo, useState } from 'react';

type JamScore = {
    jamNumber: number,
    team1Score: number,
    team2Score: number,
};

type CumulativeScoreUpdateBody = {
    jamScores: JamScore[];
};

type CumulativeScoreUpdate = {
    dataType: "CumulativeScore",
    body: CumulativeScoreUpdateBody;
};

export const CumulativeScoreGraph = () => {

    const { useDarkTheme } = useDarkThemeContext();
    const [data, setData] = useState<JamScore[]>([]);

    const socket = useMemo(() => new WebSocket(`ws://${location.hostname}:${location.port}/ws`), []);

    socket.addEventListener('open', () => {
        socket.send(JSON.stringify({
            messageType: "Subscribe",
            dataType: "CumulativeScore"
        }));
    });
    
    socket.addEventListener('message', (event) => {
        console.log(`Data received: ${JSON.stringify(event.data)}`);
        const update: CumulativeScoreUpdate = JSON.parse(event.data);
        setData(update.body.jamScores);
    })

    const team1Color = useDarkTheme ? '#ffddaa' : '#ff4400';

    return (
        <GraphContainer aspectRatio={.5}>
            <LineChart data={data}>
                <Line type="monotone" dot={false} stroke={team1Color} strokeWidth={3} dataKey="team1Score" name="White" />
                <Line type="monotone" dot={false} stroke='#00bb22' strokeWidth={3} dataKey="team2Score" name="Black" />
                <XAxis dataKey="jamNumber" name="Jam">
                    <Label value="Jam #" position="insideBottom" offset={-1} />
                </XAxis>
                <YAxis>
                    <Label value="Cumulative score" angle={-90} />
                </YAxis>
                <Tooltip />
            </LineChart>
        </GraphContainer>
    );
}