// eslint-disable-next-line no-undef
module.exports = function (api) {
	api.cache(true);
	return {
		presets: [
			'@babel/preset-typescript',
			'babel-preset-expo',
		],
		plugins: [
			'react-native-reanimated/plugin',
			[
				'module-resolver',
				{
					root: [
						'./src',
						'../lembas-api'
					],
					extensions: ['.ios.js', '.android.js', '.js', '.ts', '.tsx', '.json'],
				}
			]
		],
	};
};
