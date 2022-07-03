/** @jsxImportSource @emotion/react */
import { useEffect, useState } from 'react'
import { css } from '@emotion/react'
import Dialog from '@mui/material/Dialog'
import DialogTitle from '@mui/material/DialogTitle';
import DialogContent from '@mui/material/DialogContent';
import DialogContentText from '@mui/material/DialogContentText';
import DialogActions from '@mui/material/DialogActions';
import Button from '@mui/material/Button'
import Radio from '@mui/material/Radio';
import RadioGroup from '@mui/material/RadioGroup';
import Checkbox from '@mui/material/Checkbox';
import FormControlLabel from '@mui/material/FormControlLabel';
import FormControl from '@mui/material/FormControl';
import FormGroup from '@mui/material/FormGroup'
import FormLabel from '@mui/material/FormLabel';
import Typography from '@mui/material/Typography';
import TextField from '@mui/material/TextField'
import Stack from '@mui/material/Stack'

import Dictionaries from '../Dictionaries'


export default function Settings(props) {
    const [rpId, setRpId] = useState('')
    const [rpName, setRpName] = useState('')
    const [keyType, setKeyType] = useState('')
    const [alg, setAlg] = useState(0)
    const [authenticatorAttachment, setAuthenticatorAttachment] = useState("platform")
    const [residentKey, setResidentKey] = useState('discouraged')
    const [userVerification, setUserVerification] = useState('discouraged')
    const [origin, setOrigin] = useState('')
    const [attestationConveyancePreference, setAttestationConveyancePreference] = useState("direct")
    const [timeout, setTimeout] = useState(0)
    const [authenticatorTransports, setAuthenticatorTransports] = useState({ 'usb': false, 'nfc': false, 'ble': false, 'internal': false })
    const [validateSignCount, setValidateSignCount] = useState(false)
    const [policy, setPolicy] = useState({})
    const [usb, setUsb] = useState(false)
    const [nfc, setNfc] = useState(false)
    const [ble, setBle] = useState(false)
    const [internal, setInternal] = useState(false)

    useEffect(() => {
        const response = fetch('/policy', {
            method: 'GET',
            credentials: 'include',
            headers: {
                'Content-Type': 'application/json'
            }
        }).then(response => {
            return response.json()
        }).then(policy => {
            console.log('Returned policy', policy)
            setPolicy(policy)
            setRpId(policy.rp_id)
            setRpName(policy.rp_name)
            setKeyType(policy.key_type)
            setAlg(policy.alg)
            setAuthenticatorAttachment(policy.authenticator_attachment)
            setResidentKey(policy.resident_key)
            setUserVerification(policy.user_verification)
            setOrigin(policy.origin)
            setAttestationConveyancePreference(policy.attestation)
            setTimeout(policy.timeout)
            if (policy.hasOwnProperty('authenticator_transports')) {
                if (policy.authenticator_transports.indexOf('usb') >= 0) { authenticatorTransports.usb = true }
                if (policy.authenticator_transports.indexOf('nfc') >= 0) { authenticatorTransports.nfc = true }
                if (policy.authenticator_transports.indexOf('ble') >= 0) { authenticatorTransports.ble = true }
                if (policy.authenticator_transports.indexOf('internal') >= 0) { authenticatorTransports.internal = true }
                setAuthenticatorTransports(authenticatorTransports)
            }

        }).catch(err => {
            console.log("Error fetching policy", err)
        })

    }, [])

    const handleClose = () => {
        props.onClose()
    }

    const handleUpdate = () => {
        props.onClose()
    }

    const handleUserVerification = (event) => {
        setUserVerification(event.target.value);
    };

    const handleAuthenticatorAttachment = (event) => {
        setAuthenticatorAttachment(event.target.value);
    };

    const handleAttestationConveyancePreference = (event) => {
        setAttestationConveyancePreference(event.target.value);
    };

    const handleAuthenticatorTransports = (event) => {
        /*
        if (event.target.name === 'usb') {
            setUsb(event.target.checked)
        } else if (event.target.name === 'nfc') {
            setNfc(event.target.checked)
        } else if (event.target.name === 'ble') {
            setBle(event.target.checked)
        } else if (event.target.name === 'internal') {
            setInternal(event.target.checked)
        }
        */
        authenticatorTransports[event.target.name] = event.target.checked
        console.log(authenticatorTransports)
        setAuthenticatorTransports(authenticatorTransports)
    }

    const styles = {
        header_css: css`padding-bottom: 1.2em;`,
        textbox_css: css`width: 400px; margin-bottom: 1.2em;margin-top: 1.2em;`,
    }

    return (
        <Dialog onClose={handleClose} fullWidth
            maxWidth="lg" open={props.open}>
            <DialogTitle>Subscribe</DialogTitle>
            <DialogContent>
                <DialogContentText css={styles.header_css} >
                    Change the setting to see how it impact your registration
                </DialogContentText>
                <FormGroup>
                    <TextField
                        css={styles.textbox_css}
                        required
                        id="rp_id"
                        label="RP ID"
                        size="small"
                        value={rpId}
                    />

                    <TextField
                        css={styles.textbox_css}
                        required
                        id="rp_name"
                        label="RP Name"
                        size="small"
                        value={rpName}
                    />

                    <TextField
                        css={styles.textbox_css}
                        required
                        id="origin"
                        label="Origin"
                        size="small"
                        value={origin}
                    />

                    <FormControl>
                        <FormLabel id="demo-row-radio-buttons-group-label">User Verification</FormLabel>
                        <RadioGroup
                            row
                            aria-labelledby="user-verification-radio-buttons"
                            name="user-verification-radio-buttons-group"
                            value={userVerification}
                            onChange={handleUserVerification}
                        >
                            {Object.values(Dictionaries.UserVerificationRequirement).map(value =>
                                <FormControlLabel key={value} value={value} control={<Radio />} label={value} />
                            )}
                        </RadioGroup>
                    </FormControl>

                    <FormControl>
                        <FormLabel id="demo-row-radio-buttons-group-label">Authenticator Attachment</FormLabel>
                        <RadioGroup
                            row
                            aria-labelledby="authenticator-attachment-radio-buttons"
                            name="authenticator-attachment-radio-buttons-group"
                            value={authenticatorAttachment}
                            onChange={handleAuthenticatorAttachment}
                        >
                            {Object.values(Dictionaries.AuthenticatorAttachment).map(value =>
                                <FormControlLabel key={value} value={value} control={<Radio />} label={value} />
                            )}
                        </RadioGroup>
                    </FormControl>

                    <FormControl>
                        <FormLabel id="demo-row-radio-buttons-group-label">Attestation Conveyance Preference</FormLabel>
                        <RadioGroup
                            row
                            aria-labelledby="attestation-radio-buttons"
                            name="attestation-radio-buttons-group"
                            value={attestationConveyancePreference}
                            onChange={handleAttestationConveyancePreference}
                        >
                            {Object.values(Dictionaries.AttestationConveyancePreference).map(value =>
                                <FormControlLabel key={value} value={value} control={<Radio />} label={value} />
                            )}
                        </RadioGroup>
                    </FormControl>
                    <TextField
                        css={styles.textbox_css}
                        required
                        id="timout"
                        label="Timeout"
                        size="small"
                        value={timeout}
                    />
                    <FormControl>
                        <FormLabel >Authenticator Transports</FormLabel>
                        <FormGroup row>
                            <FormControlLabel
                                control={<Checkbox
                                    name="usb"
                                    checked={authenticatorTransports.usb}
                                    onChange={handleAuthenticatorTransports}
                                />}
                                label="USB" />
                            <FormControlLabel
                                control={<Checkbox
                                    name="nfc"
                                    checked={authenticatorTransports.nfc}
                                    onChange={handleAuthenticatorTransports}
                                />}
                                label="NFC" />
                            <FormControlLabel
                                control={<Checkbox
                                    name="ble"
                                    checked={authenticatorTransports.ble}
                                    onChange={handleAuthenticatorTransports}
                                />}
                                label="BLE" />
                            <FormControlLabel
                                control={<Checkbox
                                    name="internal"
                                    checked={authenticatorTransports.internal}
                                    onChange={handleAuthenticatorTransports}
                                />}
                                label="Internal" />
                        </FormGroup>
                    </FormControl>

                </FormGroup>
                <DialogActions>
                    <Button onClick={handleClose}>Cancel</Button>
                    <Button onClick={handleUpdate}>Update</Button>
                </DialogActions>
            </DialogContent>
        </Dialog>
    )
}

