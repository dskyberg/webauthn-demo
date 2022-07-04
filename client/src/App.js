import { useState } from 'react'
import { useStore } from './store';
import { observer } from 'mobx-react-lite'
import { Box, Flex } from '@chakra-ui/react'
import TopAppBar from './components/TopAppBar'
import Login from './components/Login'
import User from './components/User'
import Home from './components/Home'
import Policy from './components/Policy'

const App = observer(() => {
  const { settings } = useStore()
  const [settingsOpen, setSettingsOpen] = useState(false)

  const { isLoggedIn, user } = settings;

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
    <Flex>
      <TopAppBar onSettingsOpen={handleSettingsOpen} />
      <Box width="100%" as="main" mt="20">
        {isLoggedIn ? <User onLogout={logout} user={user} />
          : <Login />
        }
      </Box>
      <Policy open={settingsOpen} onClose={handleSettingsClose} />
    </Flex >
  );
})
export default App