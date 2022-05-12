/** @jsxImportSource @emotion/react */
import { css } from '@emotion/react'
import { Button, Container, Stack, Typography } from "@mui/material";

const styles = {
    button: css`
    width: 300px;
    `,
}
export default function Home(props) {
    const { onLogin, onRegister } = props

    return (
        <Container>
            <Stack alignItems="center" spacing={2}>
                <Typography variant="h1">WebAuthn Demo</Typography>
                <Button variant="contained" css={styles.button} >Login</Button>
                <Button variant="contained" css={styles.button} color="secondary">Register</Button>
            </Stack>
        </Container>
    )
}