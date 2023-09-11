import { useDarkThemeContext } from '../../../contexts/';
import { GraphContainer } from '../../atoms/GraphContainer/GraphContainer';
import { useState } from 'react';
import { useStateSocket } from '../../../hooks';
import { PenaltyLine } from '../../atoms';

interface IIndexer<K, V> {
    [key: K]: V
}

class PenaltyCounts implements IIndexer<string, number> {
    A: number;
    B: number;
    D: number;
    E: number;
    F: number;
    G: number;
    I: number;
    L: number;
    M: number;
    O: number;
    P: number;
    X: number;
    Z: number;

    constructor() {
        this.A = 0;
        this.B = 0;
        this.D = 0;
        this.E = 0;
        this.F = 0;
        this.G = 0;
        this.I = 0;
        this.L = 0;
        this.M = 0;
        this.O = 0;
        this.P = 0;
        this.X = 0;
        this.Z = 0;
    }

    [code: string]: number;
};

class CountsByTeam implements IIndexer<number, PenaltyCounts> {
    1: PenaltyCounts;
    2: PenaltyCounts;

    [key: number]: PenaltyCounts;
};

type UpdateBody = {
    penaltyCountsByTypeByTeam: CountsByTeam;
};

type PenaltiesUpdate = {
    dataType: "PenaltiesByType",
    body: UpdateBody;
};

export const TeamPenaltiesChart = () => {

    const { useDarkTheme } = useDarkThemeContext();
    const [penaltyCountsByTypeByTeam, setPenaltyCountsByTypeByTeam] = useState<CountsByTeam>({1: new PenaltyCounts(), 2: new PenaltyCounts()});

    useStateSocket<PenaltiesUpdate>("PenaltiesByType", update => {
        console.log(update);
        setPenaltyCountsByTypeByTeam(update.body.penaltyCountsByTypeByTeam);
    });

    const team1Color = useDarkTheme ? '#ffddaa' : '#ff4400';

    const getTeamTotal = (teamId: number) => 
        Object.values(penaltyCountsByTypeByTeam[teamId]).reduce((c, v) => c + v, 0);

    return (
        <GraphContainer aspectRatio={.5}>
            <div>
                <PenaltyLine team1Color={team1Color} team2Color="#0b2" team1Count={getTeamTotal(1)} team2Count={getTeamTotal(2)} title="Total" />
                {
                    Object.keys(penaltyCountsByTypeByTeam[1])
                        .map(k => ({
                            code: k,
                            team1Count: penaltyCountsByTypeByTeam[1][k],
                            team2Count: penaltyCountsByTypeByTeam[2][k]
                        }))
                        .map(line => (
                            <PenaltyLine team1Color={team1Color} team2Color="#00bb22" team1Count={line.team1Count} team2Count={line.team2Count} title={line.code} />
                        ))
                }
            </div>
        </GraphContainer>
    );
}