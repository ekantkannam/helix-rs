import pandas as pd
import seaborn as sns
import matplotlib.pyplot as plt

# Load your export
df = pd.read_csv('helix_export.tsv', sep='\t')

# Visualize the relationship
sns.lmplot(x='GC_Pct', y='ORFs_Found', data=df)
plt.title('GC Content vs. ORF Count')
plt.show()
