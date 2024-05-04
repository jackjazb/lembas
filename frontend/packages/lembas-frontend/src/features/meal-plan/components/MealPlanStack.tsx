import { createNativeStackNavigator } from '@react-navigation/native-stack';
import { MealPlanView } from 'features/meal-plan/components/MealPlanView';
import { EditShoppingListView } from 'features/meal-plan/components/EditShoppingListView';
import { Button } from 'react-native-paper';
import { StackActions } from '@react-navigation/native';
import { ShoppingListView } from 'features/meal-plan/components/ShoppingListView';

/**
 * Defines which parameters can be passed to each route.
 */
export type RecipeStackParamList = {
	'Calendar': undefined;
	'Shopping List': { from: string; to: string; };
	'Shopping List View': undefined;
};

const Stack = createNativeStackNavigator<RecipeStackParamList>();

export default function MealPlanStack(): JSX.Element {
	return (<Stack.Navigator
		initialRouteName='Calendar'
		screenOptions={{
			headerTitleStyle: { fontWeight: 'bold' }
		}}>

		<Stack.Screen
			name='Calendar'
			component={MealPlanView}
			options={{
				title: 'Meal Plan',
			}}

		/>

		<Stack.Screen
			name='Shopping List'
			component={EditShoppingListView}
			options={({ navigation }) => ({
				animation: 'fade',
				title: 'Edit Shopping List',
				headerRight: () => <>
					<Button onPress={() => navigation.dispatch(StackActions.push('Shopping List View'))}>Confirm</Button>
				</>
			})}
		/>
		<Stack.Screen
			name='Shopping List View'
			component={ShoppingListView}
			options={{
				animation: 'fade',
				title: 'Shopping List'
			}}
		/>
	</Stack.Navigator >
	);
}

