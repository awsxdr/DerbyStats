import styles from './PenaltyLine.module.scss';

interface IPenaltyLineProps {
    team1Color: string;
    team2Color: string;
    team1Count: number;
    team2Count: number;
    title: string;
}

export const PenaltyLine = ({ team1Color, team2Color, team1Count, team2Count, title}: IPenaltyLineProps) => {

    const totalCount = Math.max(1, team1Count + team2Count);

    return (
        <div className={styles.penaltyLine}>
            <span className={styles.title}>{title}</span>
            <span className={styles.percentageContainer}>
                <span style={{ backgroundColor: team1Color, width: `${team1Count / totalCount * 100}%`}}></span>
                <span style={{ backgroundColor: team2Color, width: `${team2Count / totalCount * 100}%`}}></span>
            </span>
        </div>
    );
}