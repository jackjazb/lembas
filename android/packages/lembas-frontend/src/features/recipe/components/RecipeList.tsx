import { RefreshControl, ScrollView, View } from 'react-native';
import { AnimatedFAB, Divider, Text } from 'react-native-paper';
import { useCallback, useEffect, useState } from 'react';
import { StackActions, useNavigation } from '@react-navigation/native';
import { Recipe } from 'lembas-api';
import { useAppDispatch, useAppSelector } from 'app/hooks';
import { RequestStatus } from 'app/redux-utils';
import { Loading } from 'features/common/components/Loading';
import RecipeCard from 'features/recipe/components/RecipeCard';
import { recipeActions } from 'features/recipe/recipeSlice';
import { EditMode, editRecipeActions } from 'features/recipe/editRecipeSlice';
import { CentredText } from 'features/common/components/CentredText';

/**
 * Fetches a list of recipes and renders them in RecipeCard components
 */
export function RecipeList(): JSX.Element {
	const navigation = useNavigation();
	const dispatch = useAppDispatch();

	const [pallette, setPallette] = useState([]);
	const [showFab, setShowFab] = useState(true);

	const { status, recipes } = useAppSelector(state => state.recipes);
	const refreshing = status === RequestStatus.Loading;

	useEffect(() => {
		if (status === RequestStatus.Idle) {
			dispatch(recipeActions.syncRecipes());
		}
	}, [status, dispatch]);

	const onRefresh = useCallback(() => {
		dispatch(recipeActions.syncRecipes());
	}, []);

	// Used to colour recipe cards.
	const fetchRandomPallette = async () => {
		const res = await fetch(`https://www.thecolorapi.com/scheme?hex=FAC898&count=${recipes.length}`);
		const json = await res.json();
		const hex = json.colors.map(color => color.hex.value);
		setPallette(hex);
	};

	useEffect(() => {
		fetchRandomPallette();
	}, [recipes]);


	// Hide the FAB is at the bottom of the screen.
	const onScroll = ({ layoutMeasurement, contentOffset, contentSize }) => {
		if (Math.ceil(layoutMeasurement.height + contentOffset.y) >= contentSize.height - 20) {
			setShowFab(false);
		}
		else {
			setShowFab(true);
		}
		return;
	};

	let content: JSX.Element;
	let containerStyle = {};

	const loading = status === RequestStatus.Loading && recipes.length === 0 || pallette.length === 0;

	// Only show a loading indicator on first load.
	if (loading) {
		containerStyle = { flex: 1 };
		content = <Loading />;
	}
	else if (status === RequestStatus.Failed) {
		containerStyle = { flex: 1 };
		content = (
			<CentredText >
				Network request failed
			</CentredText>
		);
	}
	else if (status === RequestStatus.Succeeded && recipes.length === 0) {
		containerStyle = { flex: 1 };
		content = (
			<CentredText>
				Tap <Text style={{ fontWeight: 'bold' }}>+</Text> to create a recipe
			</CentredText>
		);
	}
	else {
		content = (
			<View>
				<View style={{ margin: 10, gap: 10 }}>
					{
						// Render each recipe as a RecipeCard
						recipes.map((recipe: Recipe, i: number) =>
							<RecipeCard recipe={recipe} key={i} color={pallette[i]} />
						)
					}
				</View>
				<Divider style={{ marginTop: 20, marginHorizontal: 10 }} />
				<Text variant="labelMedium" style={{ textAlign: 'center', marginVertical: 10 }}>No more recipes</Text>
			</View>
		);
	}

	const refreshControl = <RefreshControl refreshing={refreshing} onRefresh={onRefresh} />;

	return (
		<View style={{ flex: 1 }}>
			<ScrollView
				contentContainerStyle={containerStyle}
				refreshControl={refreshControl}
				onScroll={(event) => onScroll(event.nativeEvent)}
			>
				{content}
			</ScrollView>
			<AnimatedFAB
				visible={showFab}
				disabled={!showFab}
				icon='plus'
				style={[{
					position: 'absolute',
					margin: 16,
					right: 0,
					bottom: 0,
				}]}
				onPress={() => {
					dispatch(editRecipeActions.setEditMode(EditMode.Create));
					dispatch(editRecipeActions.clearRecipe());
					navigation.dispatch(StackActions.push('Create Recipe'));
				}} label={''} extended={false} />
		</View >
	);
}
