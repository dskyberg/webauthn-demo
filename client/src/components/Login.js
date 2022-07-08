import React, { useState } from 'react'
import { useNavigate, useLocation } from 'react-router-dom'
import { useAuth } from '../auth'
import { Container, HStack, Button, VStack, Input, InputGroup, InputRightElement, IconButton, useToast, FormControl, FormLabel } from '@chakra-ui/react'
import { checkUser } from '../webauthn'
import { BiCaretRight } from 'react-icons/bi'
import { setWarning, setError } from './toast'

export default function Login(props) {
    let navigate = useNavigate();
    let location = useLocation();
    let auth = useAuth();

    const toast = useToast()
    const [name, setName] = useState('')
    const [displayName, setDisplayName] = useState('')

    let from = location.state?.from?.pathname || "/";

    const onCheck = () => {
        if (name === "") {
            setWarning(toast, 'Please enter a username')
            return
        }
        checkUser({ name }).then(response => {
            if (response === null) {
                setWarning(toast, "The user was not found.  Maybe register?")

            } else {
                setDisplayName(response.displayName)
                onLogin()
            }
        }).catch(error => setError(toast, error.message))
    }

    const onLogin = () => {
        if (name === "") {
            setWarning(toast, 'Please enter a username')
            return
        }
        auth.signin(name)
            .then(response => {
                console.log('login succeeded', response)
                navigate(from, { replace: true });

            }).catch(error => {
                console.log("Error:", error)
                setError(toast, error.message)
            })
    }

    const handleClickRegister = () => {
        navigate("/register", { replace: true })
    }

    return (
        <Container>
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
                    <HStack width={'100%'} d="flex" justifyContent={"right"}>
                        <Button variant="link" onClick={handleClickRegister}>Register a new user</Button>
                    </HStack>
                </FormControl>
            </VStack>
        </Container>
    )
}
