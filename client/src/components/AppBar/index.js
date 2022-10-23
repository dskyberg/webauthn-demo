import * as React from 'react';
import { useLocation, useNavigate } from 'react-router-dom'
import { observer } from 'mobx-react-lite'
import { useAuth } from '../../auth'
import { Flex, Spacer, Button, IconButton, Text, Menu, MenuItem, MenuList, MenuButton } from '@chakra-ui/react';

import DrawerButton from './DrawerButton'
import { CgProfile } from 'react-icons/cg'
import { HiHome } from 'react-icons/hi'

const AppBar = observer((props) => {
    let location = useLocation();
    let navigate = useNavigate();
    const { drawerBtnRef, onDrawerBtnClick } = props
    const auth = useAuth()
    const { isLoggedIn } = auth;

    const handleLogout = () => {
        auth.signout().then(() => {
            console.log('Logging out')
        })
    }

    const handleLogin = () => {
        navigate("/login")
    }

    const handleHomeClicked = () => {
        navigate('/')
    }

    const doLogin = () => (
        <Button size="sm" color="white" variant='ghost' onClick={handleLogin}>Sign In</Button>
    )
    const doLogout = () => {
        const handleClick = () => {
            navigate("/user")
        }
        return (
            <Menu>
                <MenuButton as={Button} variant="ghost" color="current" size="sm">
                    <CgProfile color="white" />
                </MenuButton>
                <MenuList>
                    <MenuItem onClick={handleClick}>Profile</MenuItem>
                    <MenuItem onClick={handleLogout}>Logout</MenuItem>
                </MenuList>
            </Menu >
        )
    }

    return <Flex
        as="header"
        position="fixed"
        w="100%"
        h="50px"
        alignItems="center"
        backgroundColor='teal.500'
    >
        <DrawerButton btnRef={drawerBtnRef} color="white" onBtnClicked={onDrawerBtnClick} />
        {location.pathname !== '/' && <IconButton
            size="sm"
            fontSize="lg"
            variant="ghost"
            color="white"
            marginLeft="2"
            icon={<HiHome />}
            onClick={handleHomeClicked}
        />
        }
        <Spacer />
        <Text color="white" fontFamily={'heading'}>WebAuthn Demo</Text>
        <Spacer />
        {isLoggedIn ? doLogout() : doLogin()}
    </Flex>

})
export default AppBar

function ProfileButton() {
    let navigate = useNavigate();
    const handleClick = () => {
        navigate("/user")
    }

    return (
        <IconButton
            size="sm"
            fontSize="lg"
            variant="ghost"
            color="current"
            marginLeft="2"
            icon={<CgProfile />}
            onClick={handleClick}
        />
    )
}
