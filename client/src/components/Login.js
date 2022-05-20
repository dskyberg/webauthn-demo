
/** @jsxImportSource @emotion/react */
import React, { useState } from 'react'
import { css } from '@emotion/react'
import { useStore } from '../store'
import Container from '@mui/material/Container'
import Button from '@mui/material/Button'
import Snackbar from '@mui/material/Snackbar'
import Grid from '@mui/material/Grid'
import Client from '../webauthn/Client'
import Typography from '@mui/material/Typography'
import TextField from '@mui/material/TextField'
import Stack from '@mui/material/Stack'
import Alert from '@mui/material/Alert'

export default function Login(props) {
    const { journal } = useStore()
    const [name, setName] = useState('')
    const [username, setUsername] = useState('')
    const [webauthn] = useState(new Client())
    const [alert, setAlert] = useState({ open: false, severity: 'info', message: '' })

    const setError = (message) => {
        setAlert({ open: true, severity: "error", message })
    }

    const setWarning = (message) => {
        setAlert({ open: true, severity: "warning", message })
    }

    const setSuccess = (message) => {
        setAlert({ open: true, severity: "success", message })
    }

    const onRegister = () => {
        if (username === "") {
            setWarning('Please enter a username')
            return
        }
        if (name === "") {
            setWarning('Please enter a name')
            return
        }

        webauthn.register({ name, username }, journal).then(response => {
            setSuccess('Registration successful. Try logging in.')
        }).catch(error => setError(error.message))
    }

    const onLogin = () => {
        if (username === "") {
            setWarning('Please enter a username')
            return
        }
        webauthn.login({ username }).then(response => {
            console.log('Login response: ', response);
            if (response && response.status === "ok")
                props.onLogin({
                    username,
                });
        }).catch(error => setError(error.message))
    }

    const handleSnackbarClose = (event, reason) => {
        if (reason === 'clickaway') {
            return;
        }
        setAlert({ open: false, severity: "info", message: '' });
    };

    const styles = {
        header_css: css`margin-bottom: 1em;`,
        textbox_css: css`width: 100%`,
        button_css: css`width:150px`,
        col_css: css`padding-left: 2em; padding-right: 2em;`,
    }

    return (
        <Container>
            <Snackbar
                anchorOrigin={{ vertical: "top", horizontal: "center" }}
                open={alert.open}
                autoHideDuration={6000}
                onClose={handleSnackbarClose}
            >
                <Alert onClose={handleSnackbarClose} severity={alert.severity} sx={{ width: '100%' }}>
                    {alert.message}
                </Alert>
            </Snackbar>
            <Grid container spacing={10}>
                <Grid item xs={6}>
                    <Typography css={styles.header_css} align="center" variant="h5">Register</Typography>
                    <Stack alignItems="center" spacing={2}>
                        <TextField
                            css={styles.textbox_css}
                            required
                            id="outlined-required"
                            label="User"
                            size="small"
                            value={username}
                            onChange={e => setUsername(e.target.value)}
                        />
                        <TextField
                            css={styles.textbox_css}
                            required
                            id="outlined-required"
                            label="Name"
                            size="small"
                            value={name}
                            onChange={e => setName(e.target.value)}
                        />
                        <Button css={styles.button_css} variant="contained" onClick={onRegister}>
                            Register
                        </Button>
                    </Stack>
                </Grid>
                <Grid item xs={6}>
                    <Typography css={styles.header_css} align="center" variant="h5">Login</Typography>
                    <Stack alignItems="center" spacing={2}>
                        <TextField
                            css={styles.textbox_css}
                            required
                            id="outlined-required"
                            label="Username"
                            size="small"
                            value={username}
                            onChange={e => setUsername(e.target.value)}
                        />

                        <Button css={styles.button_css} variant="contained" onClick={onLogin}>
                            Login
                        </Button>
                    </Stack>
                </Grid>

            </Grid>

        </Container >
    )
}
