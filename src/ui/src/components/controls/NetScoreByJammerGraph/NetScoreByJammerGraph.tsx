import { XAxis, YAxis, Tooltip, BarChart, Bar, Cell } from 'recharts';

import { useDarkThemeContext } from '../../../contexts/';
import { GraphContainer } from '../../atoms/GraphContainer/GraphContainer';
import { useState } from 'react';
import { useStateSocket } from '../../../hooks';

type JammerInfo = {
    name: string,
    team: number,
    jamCount: number,
    totalScore: number,
    netScore: number,
    leadCount: number,
    meanTimeToInitial: number,
}

type UpdateBody = {
    jammers: JammerInfo[];
};

type JammerStatsUpdate = {
    dataType: "JammerStats",
    body: UpdateBody;
};

type NetScore = {
    name: string,
    team: number,
    score: number,
}

export const NetScoreByJammerGraph = () => {

    const { useDarkTheme } = useDarkThemeContext();
    const [netScoreByJammer, setNetScoreByJammer] = useState<NetScore[]>([]);

    useStateSocket<JammerStatsUpdate>("JammerStats", update => {
        setNetScoreByJammer(
            update.body.jammers.map(j => ({ 
                name: j.name,
                team: j.team,
                score: j.netScore
            }))
            .sort((a, b) => b.score - a.score)
            .sort((a, b) => a.team - b.team)
        );
    }, [setNetScoreByJammer]);

    const team1Color = useDarkTheme ? '#ffddaa' : '#ff4400';

    return (
        <GraphContainer aspectRatio={.5}>
            <BarChart data={netScoreByJammer} layout="vertical" barCategoryGap={1}>
                <XAxis type="number" />
                <YAxis type="category" dataKey="name" />
                <Tooltip />
                <Bar dataKey="score" name="White">
                    {
                        netScoreByJammer.map((count, index) => (
                            <Cell key={`cell-${index}`} fill={count.team === 1 ? team1Color : '#0b2'} />
                        ))
                    }
                </Bar>
            </BarChart>
        </GraphContainer>
    );
}