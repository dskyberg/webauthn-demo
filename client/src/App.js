import { useState } from 'react'
import { useStore } from './store';
import { observer } from 'mobx-react-lite'
import Container from '@mui/material/Container'
import Box from '@mui/material/Box'
import TopAppBar from './components/TopAppBar'
//import MyFooter from './components/Footer'
import Login from './components/Login'
import User from './components/User'
import Settings from './components/Settings'
import Home from './components/Home'

const App = observer(() => {
  const { settings } = useStore()
  const [user, setUser] = useState(null)
  const [settingsOpen, setSettingsOpen] = useState(false)

  const { isLoggedIn } = settings;

  const login = (user) => {
    setUser(user)
    settings.setIsLoggedIn(true)
  }

  const logout = () => {
    settings.setIsLoggedIn(false)
  }

  const handleSettingsClose = () => {
    setSettingsOpen(false)
  }

  const handleSettingsOpen = () => {
    setSettingsOpen(true)
  }

  return (
    <main>
      <TopAppBar onSettingsOpen={handleSettingsOpen} />
      <Container >
        {isLoggedIn ? <User onLogout={logout} user={user} />
          : <Login onLogin={login} />
        }
      </Container>
      <Settings open={settingsOpen} onClose={handleSettingsClose} />
    </main >
  );
})
export default App