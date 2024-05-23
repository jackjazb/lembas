import { toIngredientQuantityList } from 'lembas-api';
import { ScrollView, View } from 'react-native';
import { Button } from 'react-native-paper';
import { useAppSelector } from 'app/hooks';
import { mealPlanSelectors } from 'features/meal-plan/mealPlanSlice';
import { IngredientQuantityList } from 'features/ingredient/components/IngredientQuantityList';
import { getPlaintextList } from 'features/common/utils';
import Clipboard from '@react-native-clipboard/clipboard';
import { useState } from 'react';
import { StackActions, useNavigation } from '@react-navigation/native';

export function ShoppingListView() {
	const editableList = useAppSelector(mealPlanSelectors.selectShoppingListEditable);
	const navigation = useNavigation();
	const [copied, setCopied] = useState(false);

	const list = toIngredientQuantityList(editableList);

	const onCopy = () => {
		const plaintext = getPlaintextList(list);
		Clipboard.setString(plaintext);
		setCopied(true);
	};

	const copyContents = copied ? 'Copied!' : 'Copy to Clipboard';

	return (
		<ScrollView style={{ padding: 10, flex: 1 }}>
			<View style={{ gap: 10, marginBottom: 30 }}>

				<IngredientQuantityList ingredients={list} />
				<View style={{ flexDirection: 'row', gap: 10 }}>
					<Button style={{ flex: 1 }} mode="outlined" onPress={() => navigation.dispatch(StackActions.popToTop())}>Exit</Button>
					<Button style={{ flex: 1 }} mode="contained" onPress={onCopy}>{copyContents}</Button>
				</View>
			</View>
		</ScrollView >
	);
}
