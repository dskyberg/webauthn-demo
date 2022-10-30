import React from 'react'
import { observer } from 'mobx-react-lite'
import {
    Container, Heading, Spacer, Box,
    FormControl, FormLabel, Input, Button, Radio, RadioGroup,
    CheckboxGroup, Checkbox, Stack, useCheckboxGroup,
    IconButton, Text, Flex, InputGroup, InputLeftElement, Tooltip
} from '@chakra-ui/react'
import { FaRedoAlt } from "react-icons/fa"

import { useStore } from '../store';

import Dictionaries from '../Dictionaries'



const Policy = observer(() => {
    const { policy } = useStore()
    const { isLoading } = policy


    const handleUpdate = () => {
        console.log("Updating policy")
        policy.patch()
    }

    const handleReset = () => {
        policy.loadModel()
    }

    const handleFieldReset = event => {
        console.log('Field reset:', event.target.id)
        policy.reset(event.target.id)
    }

    if (isLoading) {
        return (
            <Container>
                <div>LOADING</div>
            </Container>
        )
    }

    return (
        <Flex direction="row" justifyContent={"center"}>
            <Stack minWidth={"500px"} direction="column" shadow="base" borderWidth="2px" borderBottomRadius={'xl'} bg={'gray.50'} p="10" spacing={2}>
                <ManagedInput
                    label="RP ID"
                    id="rpId"
                    tooltip="The rpID is always the effective domain"
                    value={policy.rpId}
                    isDirty={policy.dirty('rpId')}
                    reset={handleFieldReset}
                    onChange={event => policy.setRpId(event.target.value)}
                />
                <ManagedInput
                    label="RP Name"
                    id="rpName"
                    tooltip="The Relying Party name that will be sent"
                    value={policy.rpName}
                    onChange={event => policy.setRpName(event.target.value)}
                    isDirty={policy.dirty('rpName')}
                    reset={handleFieldReset}
                />
                <ManagedInput
                    label="Origin"
                    id="origin"
                    tooltip="The Relying Party url"
                    value={policy.origin}
                    onChange={event => policy.setOrigin(event.target.value)}
                    isDirty={policy.dirty('origin')}
                    reset={handleFieldReset}
                />
                <ManagedInput
                    label="Time Out"
                    id="timeout"
                    tooltip="A ceremony will time out at this point"
                    value={policy.timeout}
                    onChange={event => policy.setTimeout(event.target.value)}
                    isDirty={policy.dirty('timeout')}
                    reset={handleFieldReset}
                />
                <ManagedCheckbox
                    isChecked={policy.validateSignCount}
                    onChange={event => policy.setValidateSignCount(event.target.checked)}
                    label="Check Sign Count"
                    id="validateSignCount"
                    tooltip="Validation will fail if the count does not increment. Uncheck for passkey."
                    isDirty={policy.dirty('validateSignCount')}
                    reset={handleFieldReset}

                />
                <ManagedRadios
                    label="User Verification"
                    id="userVerification"
                    tooltip="Whether the authenticator should verify the user. Cannot require for passkey."
                    value={policy.userVerification}
                    values={Dictionaries.UserVerificationRequirement}
                    onChange={policy.setUserVerification}
                    isDirty={policy.dirty('userVerification')}
                    reset={handleFieldReset}
                />

                <ManagedRadios
                    label="Authenticator Attachment"
                    id="authenticatorAttachment"
                    tooltip="Whether the authenticator can be roamable"
                    value={policy.authenticatorAttachment}
                    values={Dictionaries.AuthenticatorAttachment}
                    onChange={policy.setAuthenticatorAttachment}
                    isDirty={policy.dirty('authenticatorAttachment')}
                    reset={handleFieldReset}
                />

                <ManagedRadios
                    label="Attestation Conveyance Preference"
                    id="attestation"
                    tooltip="How the authenticator should manage attestations."
                    value={policy.attestation}
                    values={Dictionaries.AttestationConveyancePreference}
                    onChange={policy.setAttestation}
                    isDirty={policy.dirty('attestation')}
                    reset={handleFieldReset}
                />
                <ManagedCheckboxes
                    tooltip="Not implemented yet"
                    label="Authenticator Transports"
                    id="authenticatorTransports"
                    values={Dictionaries.AuthenticatorTransport}
                />
                <Stack direction="row">
                    <Button size="sm" mr="1.2rem" onClick={policy.updateFromModel}>Reset</Button>
                    <Button size="sm" onClick={handleUpdate} colorScheme="blue">Update</Button>
                </Stack>
            </Stack>
            <Box w={"20px"} />
            <Stack minWidth={"500px"} direction="column" shadow="base" borderWidth="2px" borderBottomRadius={'xl'} bg={'gray.50'} p="10" spacing={2}>
                <Heading as="h4">Templates</Heading>
            </Stack>
        </Flex>
    )
})
export default Policy


function ManagedCheckbox(props) {
    const { id, label, tooltip, isDirty, reset, ...rest } = props
    return (
        <FormControl mt="1.2rem">
            <Tooltip label={tooltip}>
                <Stack direction="row">
                    {isDirty && <FaRedoAlt id={id} onClick={reset} />}
                    <Checkbox {...rest}>{label}</Checkbox>
                </Stack>
            </Tooltip>
        </FormControl>
    )
}


function ManagedCheckboxes(props) {
    const { label, values, tooltip, onChange } = props
    const { value, getCheckboxProps } = useCheckboxGroup()

    return (
        <FormControl mt="1.2rem">
            <Tooltip label={tooltip}>
                <FormLabel>{label}</FormLabel>
            </Tooltip>
            <CheckboxGroup colorScheme='green' value={value} onChange={event => console.log("ManagedCheckbox:", event)}>
                <Stack spacing={[1, 5]} direction={['column', 'row']}>
                    {
                        Object.entries(values).map(([key, val]) =>
                            <Checkbox key={val} {...getCheckboxProps({ value: val })}>{key}</Checkbox>
                        )
                    }
                </Stack>
            </CheckboxGroup>
        </FormControl>
    )
}

function ManagedInput(props) {
    const { id, tooltip, label, isDirty, reset, ...rest } = props
    return (
        <FormControl mt="1.2rem">
            <Tooltip label={tooltip}>
                <FormLabel>{label}</FormLabel>
            </Tooltip>
            <InputGroup>
                {isDirty && <InputLeftElement children={<FaRedoAlt id={id} onClick={reset} />} />}
                <Input id={id} {...rest} />
            </InputGroup>
        </FormControl>
    )
}

function ManagedRadios(props) {
    const { id, tooltip, isDirty, reset, label, value, values, onChange } = props
    const formLabel = isDirty ? <Flex><FaRedoAlt id={id} onClick={reset} /><Text ml="1rem">{label}</Text></Flex> : label
    return (
        <FormControl mt="1.2rem">
            <Tooltip label={tooltip}>
                <FormLabel id={`${id}-label`}>{formLabel}</FormLabel>
            </Tooltip>
            <RadioGroup
                aria-labelledby={id}
                name={id}
                value={value}
                onChange={onChange}
            >
                {Object.entries(values).map(([key, val]) =>
                    <Radio key={val} mr="1rem" value={val}>{key}</Radio>
                )}
            </RadioGroup>
        </FormControl>

    )
}

