import React, { useState } from 'react'
import { useNavigate, useLocation } from 'react-router-dom'
import { useAuth } from '../auth'
import { Container, HStack, Button, VStack, Input, InputGroup, InputRightElement, IconButton, useToast, FormControl, FormLabel } from '@chakra-ui/react'
import { BiCaretRight } from 'react-icons/bi'
import { setWarning, setError } from '../components/toast'

// Will return true if the name exists.  Else false
export async function checkUser(formBody) {
    const response = await fetch('/api/users/check', {
        method: 'POST',
        credentials: 'include',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(formBody)
    })

    if (response.status === 404) {
        console.log('checkUser - not found. Returning null')
        return false
    }

    if (response.status < 200 || response.status > 205) {
        throw new Error('Server responded with error.')
    }

    return true
}


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
            if (response === false) {
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

    const handleKeyPress = e => {
        console.log('keypress')
        if (e.keyCode === 13) {
            onCheck()
        }
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
                            autoFocus={true}
                            onChange={e => setName(e.target.value)}
                            onKeyDown={handleKeyPress}
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
