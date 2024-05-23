import React from 'react';
import { createNativeStackNavigator } from '@react-navigation/native-stack';
import { TextInput } from 'react-native';
import { Appbar } from 'react-native-paper';
import { StackActions } from '@react-navigation/native';
import { IngredientView } from 'features/ingredient/components/IngredientView';
import { IngredientSearch } from 'features/ingredient/components/IngredientSearch';
import { CreateIngredient } from 'features/ingredient/components/CreateIngredient';

const Stack = createNativeStackNavigator();

export function IngredientStack(): JSX.Element {
	return (<Stack.Navigator
		initialRouteName='IngredientList'
		screenOptions={{
			headerTitle: 'Ingredients',
			headerTitleStyle: { fontWeight: 'bold' }
		}}>
		<Stack.Screen
			name='IngredientView'
			component={IngredientView}
			options={({ navigation }) => ({
				animation: 'fade',
				headerRight:
					() => <Appbar.Action icon='magnify' onPress={() => navigation.dispatch(StackActions.push('IngredientSearch'))} />
			})}
		/>
		<Stack.Screen
			name='Create Ingredient'
			component={CreateIngredient}
			options={{
				headerTitle: 'Create Ingredient',
				animation: 'fade',
			}}
		/>
		<Stack.Screen
			name='IngredientSearch'
			component={IngredientSearch}
			options={{
				animation: 'fade',
				headerTitle:
					() => (
						<TextInput
							placeholder="Search"
							onChangeText={() => { }}
							value={''}
						/>
					)
			}}
		/>

	</Stack.Navigator >
	);
}
