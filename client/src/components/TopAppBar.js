import * as React from 'react';
import { useNavigate } from 'react-router-dom'
import { useStore } from '../store'
import { useAuth } from '../auth'
import { Flex, Spacer, Button, Text } from '@chakra-ui/react';

import DrawerButton from './DrawerButton'
import SettingsButton from './SettingsButton'

export default function TopAppBar(props) {
    let navigate = useNavigate();
    const auth = useAuth()
    const { user } = auth;

    const handleLogout = () => {
        console.log('Logging out')
        auth.signout()
    }

    const handleLogin = () => {
        console.log('Logging in')
        navigate("/login")
    }

    const doLogin = () => (<Button size="sm" color="white" variant='ghost' onClick={handleLogin}>Sign In</Button>)
    const doLogout = () => (<Button size="sm" color="white" variant='ghost' onClick={handleLogout}>Sign Out</Button>)

    return <Flex
        as="header"
        position="fixed"
        w="100%"
        h="50px"
        alignItems="center"
        backgroundColor='teal.500'
    >
        <DrawerButton color="white" />
        <Spacer />
        <Text color="white" fontFamily={'heading'}>WebAuthn Demo</Text>
        <Spacer />
        {user !== null ? doLogout() : doLogin()}
        <SettingsButton color="white" onClick={props.onSettingsOpen} />
    </Flex>

}

/*
        <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
            WebAuthn Demo
        </Typography>
        {!isLoggedIn && doLogin()}
        {isLoggedIn && doLogout()}
*/