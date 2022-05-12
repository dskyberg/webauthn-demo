import { useState } from 'react'
import Dialog from '@mui/material/Dialog'
import DialogTitle from '@mui/material/DialogTitle';
import DialogContent from '@mui/material/DialogContent';
import DialogContentText from '@mui/material/DialogContentText';
import DialogActions from '@mui/material/DialogActions';
import Button from '@mui/material/Button'
import Radio from '@mui/material/Radio';
import RadioGroup from '@mui/material/RadioGroup';
import FormControlLabel from '@mui/material/FormControlLabel';
import FormControl from '@mui/material/FormControl';
import FormGroup from '@mui/material/FormGroup'
import FormLabel from '@mui/material/FormLabel';
import Dictionaries from '../Dictionaries'


export default function Settings(props) {
    const [userVerification, setUserVerification] = useState('discouraged')
    const [authenticatorAttachment, setAuthenticatorAttachment] = useState("platform")
    const [attestationConveyancePreference, setAttestationConveyancePreference] = useState("direct")
    const [authenticatorTransport, setAuthenticatorTransport] = useState("internal")

    const handleClose = () => {
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

    const handleAuthenticatorTransport = (event) => {
        setAuthenticatorTransport(event.target.value);
    };

    return (
        <Dialog onClose={handleClose} fullWidth
            maxWidth="lg" open={props.open}>
            <DialogTitle>Subscribe</DialogTitle>
            <DialogContent>
                <DialogContentText>
                    Change the setting to see how it impact your registration.
                </DialogContentText>
                <FormGroup>
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
                            aria-labelledby="user-verification-radio-buttons"
                            name="user-verification-radio-buttons-group"
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
                            aria-labelledby="user-verification-radio-buttons"
                            name="user-verification-radio-buttons-group"
                            value={attestationConveyancePreference}
                            onChange={handleAttestationConveyancePreference}
                        >
                            {Object.values(Dictionaries.AttestationConveyancePreference).map(value =>
                                <FormControlLabel key={value} value={value} control={<Radio />} label={value} />
                            )}
                        </RadioGroup>
                    </FormControl>

                    <FormControl>
                        <FormLabel id="demo-row-radio-buttons-group-label">Authenticator Transport</FormLabel>
                        <RadioGroup
                            row
                            aria-labelledby="user-verification-radio-buttons"
                            name="user-verification-radio-buttons-group"
                            value={authenticatorTransport}
                            onChange={handleAuthenticatorTransport}
                        >
                            {Object.values(Dictionaries.AuthenticatorTransport).map(value =>
                                <FormControlLabel key={value} value={value} control={<Radio />} label={value} />
                            )}
                        </RadioGroup>
                    </FormControl>

                </FormGroup>
                <DialogActions>
                    <Button onClick={handleClose}>Cancel</Button>
                    <Button onClick={handleClose}>Subscribe</Button>
                </DialogActions>
            </DialogContent>
        </Dialog>
    )
}