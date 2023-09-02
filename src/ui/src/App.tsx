import './App.css'
import { DarkThemeContextProvider } from './contexts/DarkThemeContext'
import { Routes } from './Routes';

function App() {
  return (
    <>
      <DarkThemeContextProvider>
        <Routes />
      </DarkThemeContextProvider>
    </>
  )
}

export default App
