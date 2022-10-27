import { useState, useRef } from 'react'
import { Routes, Route } from 'react-router-dom'
import { useAuth } from './auth'
import { observer } from 'mobx-react-lite'
import { Box, Flex, useDisclosure } from '@chakra-ui/react'
import { RequireAuth } from './auth'
import AppBar from './components/AppBar'
import AppDrawer from './components/AppDrawer'
import User from './components/User'
import Login from './pages/Login'
import Register from './pages/Register'
import Home from './pages/Home'
import Users from './pages/Users'
import Policy from './pages/Policy'

const App = observer(() => {
  const [settingsOpen, setSettingsOpen] = useState(false)
  const auth = useAuth()
  const drawerBtnRef = useRef();
  const { isOpen, onOpen, onClose } = useDisclosure();


  const logout = () => {
    auth.signout()
  }


  return (
    <Flex>
      <AppBar drawerBtnRef={drawerBtnRef} onDrawerBtnClick={onOpen} />
      <Box width="100%" as="main" mt="20">
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/login" element={<Login />} />
          <Route path="/register" element={<Register />} />
          <Route path="/user" element={<RequireAuth><User onLogout={logout} /></RequireAuth>} />
          <Route path="/users" element={<Users />} />
          <Route path="/policy" element={<Policy />} />
        </Routes>
      </Box>
      <AppDrawer btnRef={drawerBtnRef} isOpen={isOpen} onClose={onClose} />
    </Flex >
  );
})
export default App