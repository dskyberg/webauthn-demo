import React, { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { Container, Button, VStack, Input, useToast, FormControl, FormLabel } from '@chakra-ui/react'
import { createCredential } from '../webauthn'
import { setSuccess, setWarning, setError } from '../components/toast'

export default function Login(props) {
    let navigate = useNavigate();

    const toast = useToast()
    const [name, setName] = useState('')
    const [displayName, setDisplayName] = useState('')

    const handleClickRegister = () => {
        if (displayName === "") {
            setWarning(toast, 'Please enter a display name')
            return
        }
        if (name === "") {
            setWarning(toast, 'Please enter a username')
            return
        }

        createCredential({ name, displayName }).then(response => {
            setSuccess(toast, 'Registration successful. Try logging in.')

        }).catch(error => setError(error.message))
    }

    const handleReturnToLogin = () => {
        navigate("/login")
    }

    return (
        <Container>
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
                        id="outlined-required"
                        value={displayName}
                        onChange={e => setDisplayName(e.target.value)}
                    />
                </FormControl>
                <Button variant="solid" colorScheme="teal" bgGradient="linear(to-r, teal.400, teal.500, teal.600)" onClick={handleClickRegister}>
                    Register
                </Button>

                <Button variant="ghost" onClick={handleReturnToLogin}>
                    Return to login
                </Button>
            </VStack>
        </Container>
    )
}
