import { useCallback, useEffect, useState } from 'react';
import { Snackbar, Text, useTheme } from 'react-native-paper';
import { TextInput, View, FlatList, RefreshControl } from 'react-native';
import { Loading } from 'features/common/components/Loading';
import { useAppDispatch, useAppSelector } from 'app/hooks';
import { ingredientActions } from 'features/ingredient/ingredientSlice';
import { ingredientSearchActions } from 'features/ingredient/ingredientSearchSlice';
import { RequestStatus } from 'app/redux-utils';
import { editRecipeActions } from 'features/recipe/editRecipeSlice';
import { useNavigation } from '@react-navigation/native';
import { Ingredient } from 'lembas-api';
import { CentredText } from 'features/common/components/CentredText';
import { IngredientItem } from 'features/ingredient/components/IngredientItem';

export function SelectIngredient() {
	const dispatch = useAppDispatch();
	const navigation = useNavigation();
	const theme = useTheme();

	const [query, setQuery] = useState('');
	const isSearching = query !== '';

	const [errorSnackbarVisible, setErrorSnackbarVisible] = useState(false);

	const { recipe } = useAppSelector(state => state.editRecipe);
	const { ingredients, status } = useAppSelector(state => state.ingredients);
	const { results, status: searchStatus } = useAppSelector(state => state.ingredientSearch);

	// Set header bar contents
	useEffect(() => {
		navigation.setOptions({
			headerTitle: () => (
				<TextInput
					style={{
						fontSize: theme.fonts.titleMedium.fontSize,
						flex: 1
					}}
					placeholder="Search"
					value={query}
					onChangeText={setQuery}
				/>
			),
		});
	});

	useEffect(() => {
		if (status === RequestStatus.Idle) {
			dispatch(ingredientActions.syncIngredients());
		}

	}, [status, dispatch]);

	const onRefresh = useCallback(() => {
		dispatch(ingredientActions.syncIngredients());
		if (isSearching) {
			dispatch(ingredientSearchActions.syncResults(query));
		}
	}, [query]);


	useEffect(() => {
		if (isSearching) {
			dispatch(ingredientSearchActions.syncResults(query));
		}
	}, [query, dispatch]);

	const onConfirm = (ingredient: Ingredient) => {
		const ingredientIds = recipe.ingredients.map(i => i.ingredient.id);
		if (ingredientIds.includes(ingredient.id)) {
			setErrorSnackbarVisible(true);
		}
		else {
			dispatch(editRecipeActions.addIngredient(ingredient));
			navigation.goBack();
		}
	};


	const refreshing = status === RequestStatus.Loading;
	const refreshControl = <RefreshControl refreshing={refreshing} onRefresh={onRefresh} />;

	let content: React.ReactNode;
	const loading = status === RequestStatus.Loading || searchStatus === RequestStatus.Loading;
	if (loading) {
		content = <Loading />;
	}
	else if (status === RequestStatus.Failed) {
		content = <CentredText>Network error</CentredText>;
	}
	else if (isSearching && results.length === 0) {
		content = <Text
			variant='titleMedium'
			style={{
				margin: 20,
				justifyContent: 'center',
				alignSelf: 'center'
			}}
		>
			No matching ingredients
		</Text>;
	}
	else {
		const list = isSearching ? results : ingredients;
		content =
			<FlatList
				data={list}
				refreshControl={refreshControl}
				renderItem={({ item }) =>
					<IngredientItem
						ingredient={item}
						onPress={() => onConfirm(item)}
					/>}
				keyExtractor={(item) => item.id.toString()}
			/>;

	}

	return (
		<View style={{ flex: 1 }}>
			{content}
			<Snackbar
				style={{ backgroundColor: theme.colors.error }}
				visible={errorSnackbarVisible}
				duration={700}
				onDismiss={() => setErrorSnackbarVisible(false)}
			>
				Ingredient already in use
			</Snackbar>
		</View >
	);
}