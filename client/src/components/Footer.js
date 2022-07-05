import { Flex, Box, Container, Stack, Text } from '@chakra-ui/react'

export default function Footer() {

    return (
        <Flex
            bg={'teal.50'}
            color={'teal.700'}
            w="100%"
            h="50px"
        >

            <Box
                borderTopWidth={1}
                borderStyle={'solid'}
                borderColor={'gray.200'}>
                <Container
                    as={Stack}
                    maxW={'6xl'}
                    py={4}
                    direction={{ base: 'column', md: 'row' }}
                    spacing={4}
                    justify={{ base: 'center', md: 'space-between' }}
                    align={{ base: 'center', md: 'center' }}>
                    <Text>© 2022 Swankymutt. All rights reserved</Text>
                </Container>
            </Box>
        </Flex >
    )
}