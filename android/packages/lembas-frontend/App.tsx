import { NavigationContainer } from '@react-navigation/native';
import { SafeAreaView, StatusBar } from 'react-native';
import { MD3LightTheme, PaperProvider } from 'react-native-paper';
import { Amplify } from 'aws-amplify';
import { Authenticator } from '@aws-amplify/ui-react-native';
import React from 'react';
import { TabStack } from './src';

Amplify.configure({
	Auth: {
		Cognito: {
			userPoolId: 'eu-west-1_PWeUfVLvH',
			userPoolClientId: '7r5a7t2ugbcnj9cv1hfab85jik',

			signUpVerificationMethod: 'code',
			loginWith: {
				// Hosted UI configuration
				oauth: {
					domain: 'your_cognito_domain',

					scopes: [
						'phone',
						'email',
						'profile',
						'openid',
						'aws.cognito.signin.user.admin'
					],
					redirectSignIn: ['http://localhost:3000/'],
					redirectSignOut: ['http://localhost:3000/'],
					responseType: 'code' // or 'token', note that REFRESH token will only be generated when the responseType is code
				}
			}
		}
	}
});

function InnerApp(): JSX.Element {
	return (
		<PaperProvider theme={MD3LightTheme} >
			<NavigationContainer>
				<SafeAreaView style={{ flex: 1 }}>
					<StatusBar />
					<TabStack />
				</SafeAreaView>
			</NavigationContainer>
		</PaperProvider >

	);
}

export function AuthWrappedApp(): JSX.Element {
	return (
		<Authenticator.Provider>
			<Authenticator signUpAttributes={['name', 'email']}>
				<InnerApp />
			</Authenticator>
		</Authenticator.Provider>
	);
}
export default function App(): JSX.Element {

	return (
		<InnerApp />
		// <AuthWrappedApp />	// Uncomment this for login
	);
}
