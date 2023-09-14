import { Button, MenuItem } from "@blueprintjs/core";
import { ItemRenderer, Select } from "@blueprintjs/select";
import { useState } from "react";
import { useSearchParams } from "react-router-dom";
import { useGlobalStateSocket } from "../../../hooks/state-socket";

type Team = {
    name: string,
    color: string,
}

type Game = {
    id: string,
    isCurrent: boolean,
    homeTeam: Team,
    awayTeam: Team,
}

type GamesUpdate = {
    body: Game[];
}

export const GamesList = () => {
    const [searchParams, setSearchParams] = useSearchParams();
    const gameId = searchParams.get('gameId');

    const [games, setGames] = useState<Game[]>([]);
    const [selectedGame, setSelectedGame] = useState<Game>()

    useGlobalStateSocket<GamesUpdate>("Games", update => {
        setGames(update.body);

        const game = update.body.find(({ id }) => gameId === id);

        if(game) {
            setSelectedGame(game);
        } else {
            setSearchParams({ ...Object.fromEntries(searchParams.entries()), gameId: update.body.find(({ isCurrent }) => isCurrent)!.id });
        }
    }, [setGames, setSelectedGame, searchParams]);

    const handleItemSelect = (game: Game) => {
        setSearchParams({ ...Object.fromEntries(searchParams.entries()), gameId: game.id });
    }

    const getGameTitle = (game: Game) => `${game.homeTeam.name} vs ${game.awayTeam.name}`;

    const renderGame: ItemRenderer<Game> = (game, { handleClick, handleFocus, modifiers }) => {
        return (
            <MenuItem
                active={modifiers.active}
                disabled={modifiers.disabled}
                key={getGameTitle(game)}
                label={game.isCurrent ? ' (Current)' : ''}
                onClick={handleClick}
                onFocus={handleFocus}
                roleStructure="listoption"
                text={getGameTitle(game)}
            />
        );
    }

    return (
        <>
            <Select<Game>
                items={games}
                itemRenderer={renderGame}
                filterable={false}
                onItemSelect={handleItemSelect}
            >
                <Button text={selectedGame && getGameTitle(selectedGame)} rightIcon="double-caret-vertical" placeholder="Select game" />
            </Select>
        </>
    );
}