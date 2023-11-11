import './App.css'
import { GameContextProvider } from './contexts';
import { DarkThemeContextProvider } from './contexts/DarkThemeContext'
import { Routes } from './Routes';

function App() {
  return (
    <>
      <GameContextProvider>
        <DarkThemeContextProvider>
          <Routes />
        </DarkThemeContextProvider>
      </GameContextProvider>
    </>
  )
}

export default App
