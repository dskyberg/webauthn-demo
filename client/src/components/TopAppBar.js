import * as React from 'react';
import { useStore } from '../store'

import { Flex, Spacer, Button, Text } from '@chakra-ui/react';

import { TbSettings } from 'react-icons/tb'

import DrawerButton from './DrawerButton'
import SettingsButton from './SettingsButton'

export default function TopAppBar(props) {
    const { settings } = useStore()

    const { isLoggedIn } = settings;

    const handleLogout = () => {
        console.log('Logging out')
        settings.logout()
    }

    const handleLogin = () => {
        console.log('Logging in')
    }

    const doLogin = () => (<Button color="inherit" onClick={handleLogin}>Sign In</Button>)
    const doLogout = () => (<Button color="inherit" onClick={handleLogout}>Sign Out</Button>)

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