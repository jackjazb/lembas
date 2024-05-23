import { StackActions, useNavigation } from '@react-navigation/native';
import { AuthUser, getCurrentUser } from 'aws-amplify/auth';
import { API_URL, connectionTest, loadDemoData, } from 'lembas-api';
import { ReactNode, useEffect, useState } from 'react';
import { View } from 'react-native';
import { ActivityIndicator, Icon, List, Text } from 'react-native-paper';
import { useAppDispatch } from 'app/hooks';
import { RequestStatus } from 'app/redux-utils';
import { ingredientActions } from 'features/ingredient/ingredientSlice';
import { recipeActions } from 'features/recipe/recipeSlice';

/**
 * Renders a view for settings.
 */
export function Settings() {
	const navigation = useNavigation();
	const dispatch = useAppDispatch();

	const [currentUser, setCurrentUser] = useState<AuthUser | undefined>();
	const [apiStatus, setApiStatus] = useState<RequestStatus>(RequestStatus.Loading);

	useEffect(() => {
		async function getUser() {
			const user = await getCurrentUser();
			setCurrentUser(user);
		}
		async function apiTest() {
			const okay = await connectionTest();
			if (okay) {
				setApiStatus(RequestStatus.Succeeded);
			}
			else {
				setApiStatus(RequestStatus.Failed);
			}
		}
		getUser();
		apiTest();
	});

	let conn: JSX.Element;
	if (apiStatus === RequestStatus.Loading) {
		conn = (
			<View style={{ flexDirection: 'row', alignItems: 'center', gap: 10 }}>
				<ActivityIndicator animating={true} />
			</View >);
	}
	else if (apiStatus === RequestStatus.Succeeded) {
		conn =
			(<View style={{ flexDirection: 'row', alignItems: 'center', gap: 10 }}>
				<Icon source="check" color="green" size={26} />
			</View >);
	}
	else if (apiStatus === RequestStatus.Failed) {
		conn =
			(<View style={{ flexDirection: 'row', alignItems: 'center', gap: 10 }}>
				<Text style={{ color: 'red' }}>Connection error</Text>
			</View >);
	}

	return (
		<View style={{ margin: 10, gap: 10 }}>
			<Text variant='headlineMedium'>Developer</Text >
			<InfoItem title="API" info={API_URL} icon="glasses" right={() => conn} />
			{currentUser &&
				<InfoItem title="Current User" info={`Username: ${currentUser.username}\nSub: ${currentUser.userId}`} icon="account" />}
			<ActionItem
				title="Load Demo Data"
				action={async () => {
					await loadDemoData();
					dispatch(ingredientActions.syncUserIngredients());
					dispatch(recipeActions.syncRecipes());
					navigation.dispatch(StackActions.popToTop());
				}} icon="upload" />
		</View >
	);
}

function InfoItem(props: { title: string, info: string, icon: string; right?: () => ReactNode; }) {
	const { title, info, icon, right } = props;
	return (
		<List.Item
			title={title}
			description={info}
			left={() => <List.Icon icon={icon} />}
			right={right}
		/>
	);
}


function ActionItem(props: { title: string, action: () => void, icon: string; }) {
	const { title, action, icon } = props;
	return (
		<List.Item
			titleStyle={{}}
			onPress={() => action()}
			title={title}
			left={() => <List.Icon icon={icon} />}
		/>
	);
}