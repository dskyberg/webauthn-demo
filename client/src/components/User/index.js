import React, { useState, useEffect } from 'react';

import { Center, Container, Skeleton } from '@chakra-ui/react';
import { useAuth } from '../../auth'

import UserProfile from './UserProfile'
import UserCredential from './UserCredential'
import EmptyUser from './EmptyUser'

// Get the user associated with the name
export async function getUser(formBody) {
    const response = await fetch('/api/users', {
        method: 'POST',
        credentials: 'include',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(formBody)
    })

    if (response.status === 404) {
        console.log('getUser - not found. Returning null')
        return null
    }

    if (response.status < 200 || response.status > 205) {
        throw new Error('Server responded with error.')
    }

    return await response.json()
}

export async function getUserCredentials(formBody) {
    const response = await fetch('/api/credentials/user', {
        method: 'POST',
        credentials: 'include',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(formBody)
    })

    if (response.status === 404) {
        console.log('getUserCredentials - not found. Returning null')
        return null
    }

    if (response.status < 200 || response.status > 205) {
        throw new Error('Server responded with error.')
    }

    return await response.json()

}

export default function User(props) {
    const auth = useAuth()
    const [user, setUser] = useState(null)
    const [credentials, setCredentials] = useState(null)

    useEffect(() => {
        if (!auth.isLoggedIn) {
            console.log('User.useEffect: not logged in.')
            return;
        }
        getUser({ name: auth.user })
            .then(user => {
                console.log('Found User:', user)
                setUser(user)
                return (user)
            })
            .then(user => {
                return getUserCredentials(user)
            })
            .then(credentials => {
                console.log('Got credentials:', credentials)
                setCredentials(credentials)
            })

    }, [])

    if (user === null) {
        return (
            <EmptyUser />
        )
    }

    return (
        <Center>
            <Container m="2rem" maxWidth="800">
                <UserProfile user={user} />
                <UserCredential credential={credentials} />
            </Container >
        </Center >
    )
}
