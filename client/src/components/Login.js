
import React, { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { useStore } from '../store'
import { Container, Button, VStack, Input, Text, useToast, FormControl, FormLabel } from '@chakra-ui/react'
import { checkUser, createCredential, assertCredential } from '../webauthn'

export default function Login(props) {
    let navigate = useNavigate()
    const { settings } = useStore()
    const toast = useToast()
    const [ceremony, setCeremony] = useState('check')
    const [name, setName] = useState('')
    const [displayName, setDisplayName] = useState('')

    const showToast = (status, message) => {
        toast({
            position: 'top',
            status: status,
            description: message,
            isClosable: true,
        })

    }
    const setError = (message) => {
        console.error(message)
        showToast('error', message)
    }

    const setWarning = (message) => {
        console.warn(message)
        showToast('warning', message)
    }

    const setSuccess = (message) => {
        console.log(message)
        showToast('success', message)
    }

    const onCheck = () => {
        if (name === "") {
            setWarning('Please enter a username')
            return
        }
        checkUser({ name }).then(response => {
            if (response === null) {
                setCeremony('register')
                setWarning("Looks like a new user.  Let's register")

            } else {
                setDisplayName(response.displayName)
                setCeremony('authn')
                setSuccess(`You're already registered, ${response.displayName}. Try logging in.`)

            }
        }).catch(error => setError(error.message))
    }

    const onRegister = () => {
        if (displayName === "") {
            setWarning('Please enter a display name')
            return
        }
        if (name === "") {
            setWarning('Please enter a username')
            return
        }

        createCredential({ name, displayName }).then(response => {
            setCeremony('authn')
            setSuccess('Registration successful. Try logging in.')
        }).catch(error => setError(error.message))
    }

    const onLogin = () => {
        if (name === "") {
            setWarning('Please enter a username')
            return
        }
        assertCredential({ name })
            .then(response => {
                let json_response = JSON.parse(response)
                if (json_response.status == 'ok') {
                    settings.setUser(name)
                    settings.setIsLoggedIn(true)
                    navigate("/user", { replace: true })
                }

            }).catch(error => {
                setError(error.message)
            })
    }

    const handleSnackbarClose = (event, reason) => {
        if (reason === 'clickaway') {
            return;
        }
        //setAlert({ open: false, severity: "info", message: '' });
    };

    return (
        <Container >
            <VStack shadow="base"
                borderWidth="1px" borderBottomRadius={'xl'} bg={'gray.50'} p="10" alignItems="center" spacing={2}>
                <FormControl id="user-name">
                    <FormLabel mb='8px'>Username</FormLabel>
                    <Input
                        type="text"
                        placeholder="your.name@email.com"
                        value={name}
                        onChange={e => setName(e.target.value)}
                    />
                </FormControl>
                {(displayName !== '' || ceremony === 'register') &&
                    <FormControl id="display-name">
                        <FormLabel >Display Name</FormLabel>
                        <Input
                            placeholder="Your Name"
                            disabled={ceremony !== 'register'}
                            id="outlined-required"
                            value={displayName}
                            onChange={e => setDisplayName(e.target.value)}
                        />
                    </FormControl>
                }
                {ceremony === 'check' &&
                    <Button variant="solid" colorScheme="teal" bgGradient="linear(to-r, teal.400, teal.500, teal.600)" onClick={onCheck}>
                        Check for credentials
                    </Button>
                }
                {ceremony === 'register' &&
                    <Button variant="solid" colorScheme="teal" bgGradient="linear(to-r, teal.400, teal.500, teal.600)" onClick={onRegister}>
                        Register
                    </Button>
                }
                {ceremony === 'authn' &&
                    <Button variant="solid" colorScheme="teal" bgGradient="linear(to-r, teal.400, teal.500, teal.600)" onClick={onLogin}>
                        Login
                    </Button>
                }
            </VStack>
        </Container>
    )
}
