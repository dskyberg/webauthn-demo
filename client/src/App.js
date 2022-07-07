import { useState } from 'react'
import { Routes, Route } from 'react-router-dom'
import { useStore } from './store'
import { useAuth } from './auth'
import { observer } from 'mobx-react-lite'
import { Box, Flex, Stack } from '@chakra-ui/react'
import { RequireAuth } from './auth'
import TopAppBar from './components/TopAppBar'
import Login from './components/Login'
import User from './components/User'
import Home from './components/Home'
import Users from './components/Users'
import Policy from './components/Policy'
import Footer from './components/Footer'

const App = observer(() => {
  const auth = useAuth()
  const { settings } = useStore()
  const [settingsOpen, setSettingsOpen] = useState(false)

  const { isLoggedIn, user } = auth;

  const logout = () => {
    auth.signout()
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
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/login" element={<Login />} />
          <Route path="/user" element={<RequireAuth><User onLogout={logout} user={user} /></RequireAuth>} />
          <Route path="/users" element={<Users />} />
        </Routes>
      </Box>
      <Policy open={settingsOpen} onClose={handleSettingsClose} />
    </Flex >
  );
})
export default App