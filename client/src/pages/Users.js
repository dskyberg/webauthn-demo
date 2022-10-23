import { useEffect } from 'react';
import { observer } from 'mobx-react-lite'
import {
    Button,
    Container,
    Stack,
} from "@chakra-ui/react";

import { useStore } from '../store';

const Users = observer((props) => {
    const { onLogin, onRegister } = props
    const { users } = useStore()
    const { isLoading } = users

    useEffect(() => {
        users.loadUsers()
    }, [])

    if (isLoading) {
        return (
            <div>LOADING</div>
        )
    }

    return (
        <Container>
            <Stack spacing={2} direction="column">
                <p>WebAuthn Demo</p>

            </Stack>
        </Container>
    )
})

export default Users;