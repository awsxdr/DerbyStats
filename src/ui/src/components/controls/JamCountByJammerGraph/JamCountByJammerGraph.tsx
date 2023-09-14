import { Label, XAxis, YAxis, Tooltip, BarChart, Bar, Cell } from 'recharts';

import { useDarkThemeContext } from '../../../contexts/';
import { GraphContainer } from '../../atoms/GraphContainer/GraphContainer';
import { useState } from 'react';
import { useStateSocket } from '../../../hooks';

type JammerInfo = {
    name: string,
    team: number,
    jamCount: number,
    totalScore: number,
    meanNetPerJam: number,
    leadCount: number,
    meanTimeToInitial: number,
}

type UpdateBody = {
    jammers: JammerInfo[];
};

type JammerStatsUpdate = {
    dataType: "PenaltiesByType",
    body: UpdateBody;
};

type JamCounts = {
    name: string,
    team: number,
    count: number,
}

export const JamCountByJammerGraph = () => {

    const { useDarkTheme } = useDarkThemeContext();
    const [jamCountsByJammer, setJamCountsByJammer] = useState<JamCounts[]>([]);

    useStateSocket<JammerStatsUpdate>("JammerStats", update => {
        setJamCountsByJammer(
            update.body.jammers.map(j => ({ 
                name: j.name,
                team: j.team,
                count: j.jamCount
            }))
            .sort((a, b) => b.count - a.count)
            .sort((a, b) => a.team - b.team)
        );
    }, [setJamCountsByJammer]);

    const team1Color = useDarkTheme ? '#ffddaa' : '#ff4400';
    console.log(jamCountsByJammer);
    return (
        <GraphContainer aspectRatio={.5}>
            <BarChart data={jamCountsByJammer} layout="vertical" barCategoryGap={1}>
                <XAxis type="number" />
                <YAxis type="category" dataKey="name" />
                <Tooltip />
                <Bar dataKey="count" name="White">
                    {
                        jamCountsByJammer.map((count, index) => (
                            <Cell key={`cell-${index}`} fill={count.team === 1 ? team1Color : '#0b2'} />
                        ))
                    }
                </Bar>
            </BarChart>
        </GraphContainer>
    );
}