import { store } from 'app/store';
import { IngredientStack } from 'features/ingredient/components/IngredientStack';
import MealPlanStack from 'features/meal-plan/components/MealPlanStack';
import { Icon } from 'react-native-paper';
import { createMaterialBottomTabNavigator } from 'react-native-paper/react-navigation';

import { Provider } from 'react-redux';
import { RecipeStack } from 'features/recipe/components/RecipeStack';

const Tab = createMaterialBottomTabNavigator();

/**
 * Renders a tab navigator (the main entry point for the app's screens
 */
export function TabStack() {
	return (
		<>
			<Provider store={store}>
				<Tab.Navigator>
					<Tab.Screen name="Recipes" component={RecipeStack} options={{
						tabBarIcon: ({ focused }) => (
							<Icon source={focused ? 'food-apple' : 'food-apple-outline'} size={26} />
						),
					}}
					/>
					<Tab.Screen name="Meal Plan" component={MealPlanStack} options={{
						tabBarIcon: ({ focused }) => (
							<Icon source={focused ? 'clipboard-list' : 'clipboard-list-outline'} size={26} />
						),
					}} />
					<Tab.Screen name="Ingredients" component={IngredientStack} options={{
						tabBarIcon: ({ focused }) => (
							<Icon source={focused ? 'package-variant' : 'package-variant-closed'} size={26} />
						),
					}} />
				</Tab.Navigator>
			</Provider>
		</>
	);
}