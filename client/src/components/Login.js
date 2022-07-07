
import React, { useState } from 'react'
import { useNavigate, useLocation } from 'react-router-dom'
import { useAuth } from '../auth'
import { useStore } from '../store'
import { Container, Button, VStack, Input, InputGroup, InputRightElement, IconButton, Text, useToast, FormControl, FormLabel } from '@chakra-ui/react'
import { checkUser, createCredential, assertCredential } from '../webauthn'
import { BiCaretRight } from 'react-icons/bi'

export default function Login(props) {
    let navigate = useNavigate();
    let location = useLocation();
    let auth = useAuth();

    const { settings } = useStore()
    const toast = useToast()
    const [ceremony, setCeremony] = useState('check')
    const [name, setName] = useState('')
    const [displayName, setDisplayName] = useState('')

    let from = location.state?.from?.pathname || "/";

    const showToast = (status, message, title) => {
        toast({
            position: 'top',
            status: status,
            title: title,
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
                setWarning("The user was not found.  Maybe register?")

            } else {
                setDisplayName(response.displayName)
                onLogin()
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
            setCeremony('check')
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
                    auth.signin(name, () => {
                        navigate(from, { replace: true });
                    })
                }

            }).catch(error => {
                setError(error.message)
            })
    }

    const handleClickRegister = () => {
        setCeremony('register')
    }

    const doCeremonyCheckUser = () => {
        return (
            <VStack shadow="base"
                borderWidth="1px" borderBottomRadius={'xl'} bg={'gray.50'} p="10" alignItems="center" spacing={2}>
                <FormControl id="user-name">
                    <FormLabel mb='8px'>Username</FormLabel>
                    <InputGroup>
                        <Input
                            type="text"
                            placeholder="your.name@email.com"
                            value={name}
                            onChange={e => setName(e.target.value)}
                        />
                        <InputRightElement>
                            <IconButton onClick={onCheck}>{<BiCaretRight />}</IconButton>
                        </InputRightElement>
                    </InputGroup>
                    <Button variant="link" onClick={handleClickRegister}>Register a new user</Button>
                </FormControl>
            </VStack>
        )
    }

    const doCeremonyRegister = () => {
        return (
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
                <Button variant="solid" colorScheme="teal" bgGradient="linear(to-r, teal.400, teal.500, teal.600)" onClick={onRegister}>
                    Register
                </Button>

            </VStack>
        )
    }

    return (
        <Container >
            {ceremony === 'check' && doCeremonyCheckUser()}
            {ceremony === 'register' && doCeremonyRegister()}
        </Container>
    )
}
