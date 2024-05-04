import { Appbar, Button, IconButton } from 'react-native-paper';
import { createNativeStackNavigator } from '@react-navigation/native-stack';
import React from 'react';
import { RecipeList } from './RecipeList';
import { StackActions } from '@react-navigation/native';
import { ViewRecipe } from './ViewRecipe';
import { useAppSelector } from 'app/hooks';
import { EditRecipe } from 'features/recipe/components/EditRecipe';
import { Settings } from 'features/common/components/Settings';
import { SelectIngredient } from 'features/recipe/components/SelectIngredient';
import { useAuthenticator } from '@aws-amplify/ui-react-native';


const Stack = createNativeStackNavigator();

/**
 * This component defines the 'Recipes' screen's navigation stack. All routes available from the Recipes
 * screen should be defined here.
 */
export function RecipeStack(): JSX.Element {
	const recipe = useAppSelector(state => state.recipes.selected);
	const { signOut } = { signOut: () => { } };// useAuthenticator();

	return (
		<Stack.Navigator
			initialRouteName='Recipes'
			screenOptions={{
				headerTitleStyle: { fontWeight: 'bold' }
			}}>

			<Stack.Screen
				name='My Recipes'
				component={RecipeList}
				options={({ navigation }) => ({
					headerRight: () => <>
						<Appbar.Action
							icon='cog'
							onPress={() => navigation.dispatch(StackActions.push('Settings'))} />
					</>
				})}
			/>

			<Stack.Screen
				name='View Recipe'
				component={ViewRecipe}
				options={() => ({
					title: recipe.name,
					animation: 'fade',
				})}
			/>

			<Stack.Screen
				name='Create Recipe'
				component={EditRecipe}
				options={{
					animation: 'fade',
					headerRight: () => <IconButton icon="check" />	// Needed to prevent flicker
				}}
			/>

			<Stack.Screen
				name='Edit Recipe'
				component={EditRecipe}
				options={() => ({
					animation: 'fade',
					headerRight: () => <IconButton icon="check" />
				})}
			/>

			<Stack.Screen
				name='Select Ingredient'
				component={SelectIngredient}
				options={{
					animation: 'fade',
				}}
			/>
			<Stack.Screen
				name='Settings'
				component={Settings}
				options={() => ({
					animation: 'fade',
					headerRight: () => <>
						<Button onPress={signOut}>Sign Out</Button>
					</>
				})}
			/>
		</Stack.Navigator >
	);
}

