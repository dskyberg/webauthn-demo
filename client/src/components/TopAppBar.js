import * as React from 'react';
import { useStore } from '../store'

import AppBar from '@mui/material/AppBar';
import Box from '@mui/material/Box';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import Button from '@mui/material/Button';
import IconButton from '@mui/material/IconButton';
import MenuIcon from '@mui/icons-material/Menu';
import SettingsIcon from '@mui/icons-material/Settings'

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

    return <Box sx={{ flexGrow: 1 }}>
        <AppBar position="static">
            <Toolbar>
                <IconButton
                    size="large"
                    edge="start"
                    color="inherit"
                    aria-label="menu"
                    sx={{ mr: 2 }}
                >
                    <MenuIcon />
                </IconButton>
                <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
                    WebAuthn Demo
                </Typography>
                {!isLoggedIn && doLogin()}
                {isLoggedIn && doLogout()}
                <IconButton
                    size="large"
                    edge="start"
                    color="inherit"
                    aria-label="menu"
                    sx={{ mr: 2 }}
                    onClick={props.onSettingsOpen}
                >
                    <SettingsIcon />
                </IconButton>
            </Toolbar>
        </AppBar>
    </Box>

}